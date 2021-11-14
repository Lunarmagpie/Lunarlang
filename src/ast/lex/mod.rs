use regex::{Match, Regex};
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
pub enum TType {
    VAR,
    OPEN_PAREN,
    CLOSE_PAREN,

    // Operations
    EQUALS,
    ADD,
    SUBTRACT,
    MULTIPLY,
    DEVIDE,

    // Data Types
    STRING,
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
    token: String,
    ttype: TType,
}

struct PartialToken<'t> {
    regex_match: Match<'t>,
    ttype: TType,
}

fn clean_string<'a>(content: String) -> Vec<String> {

    let empty_line = Regex::new(r"\b").unwrap();
    let mut out: Vec<String> = Vec::new();

    for line in content.replace(";","\n").split('\n') {
        if empty_line.find(line) == None {
            continue
        };

        out.push(line.to_string())

    }

    out



}

pub fn main(content: String) -> Vec<Vec<Token>> {
    let rules = vec![
        Rule::new(r"[A-Za-z0-0_]+", TType::VAR),
        Rule::new(r"\(", TType::OPEN_PAREN),
        Rule::new(r"\)", TType::CLOSE_PAREN),
        //
        Rule::new(r"=", TType::EQUALS),
        //
        Rule::new("\".*\"", TType::STRING),
    ];

    let mut tokens: Vec<Vec<Token>> = Vec::new();

    // Split the program into lines
    for line in clean_string(content) {
        let mut matches_this_line: Vec<PartialToken> = Vec::new();

        for rule in &rules {
            for m in rule.regex.find_iter(&line) {
                matches_this_line.push(PartialToken {
                    regex_match: m,
                    ttype: rule.ttype,
                })
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
                .collect::<Vec<Token>>(),
        );
    }

    tokens
}
