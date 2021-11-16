use regex::{Match, Regex};
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TType {
    //Whitespace
    Whitespace,

    //Keywords
    OpenParen,
    CloseParen,
    Comma,
    EnterScope,
    ExitScope,
    FuncDef,

    //Misc
    Var,

    // Operations
    Equals,
    Add,
    Subtract,
    Multiply,
    Divide,

    // Data Types
    String,
    Digit,
    Float,
}

#[derive(Debug)]
pub struct Rule {
    pub regex: Regex,
    pub ttype: TType,
}

impl Rule {
    fn new(reg: &str, ttype: TType) -> Rule {
        Rule {
            regex: Regex::new(reg).unwrap(),
            ttype,
        }
    }
}

#[derive(Debug)]
pub struct Token {
    pub token: String,
    pub ttype: TType,
}

struct PartialToken<'t> {
    regex_match: Match<'t>,
    ttype: TType,
}

fn clean_string<'a>(content: String) -> Vec<String> {
    let empty_line = Regex::new(r"\b").unwrap();
    let mut out: Vec<String> = Vec::new();

    for line in content.replace(";", "\n").split('\n') {
        if empty_line.find(line) == None {
            continue;
        };

        out.push(line.to_string())
    }

    out
}

pub fn main(content: String) -> Vec<Vec<Token>> {
    let rules = vec![
        Rule::new(r"\(", TType::OpenParen),
        Rule::new(r"\)", TType::CloseParen),
        Rule::new(r",", TType::Comma),
        Rule::new(r"do", TType::EnterScope),
        Rule::new(r"end", TType::ExitScope),
        Rule::new(r"fn", TType::FuncDef),
        //
        Rule::new(r"=", TType::Equals),
        Rule::new("\".*\"", TType::String),
        Rule::new(r"\d+", TType::Digit),
        Rule::new(r"\d+\.\d+", TType::Float),
        //
        Rule::new(r"[A-Za-z0-9_]+", TType::Var),
        //Whitespace
        Rule::new(r"\s", TType::Whitespace),
    ];

    let mut tokens: Vec<Vec<Token>> = Vec::new();

    // Split the program into lines
    for (line_num, line) in clean_string(content).iter().enumerate() {
        let mut matches_this_line: Vec<PartialToken> = Vec::new();
        let mut used_chars = vec![false; line.len()];
        for rule in &rules {
            for m in rule.regex.find_iter(line) {
                if used_chars[m.start()] {
                    continue;
                }
                used_chars.splice(m.start()..m.end(), vec![true; m.end() - m.start()]);

                matches_this_line.push(PartialToken {
                    regex_match: m,
                    ttype: rule.ttype,
                })
            }
        }

        for (i, c) in used_chars.iter().enumerate() {
            if !*c {
                panic!("Token error at {}:{}", line_num, i);
            }
        }

        // Sort the tokens in the line
        matches_this_line.sort_by_key(|x| x.regex_match.start());
        tokens.push(
            matches_this_line
                .into_iter()
                .map(|x| Token {
                    token: line[x.regex_match.start()..x.regex_match.end()].to_string(),
                    ttype: x.ttype,
                })
                .filter(|x| x.ttype != TType::Whitespace)
                .collect::<Vec<Token>>(),
        );
    }

    tokens
}
