use crate::token::{Token, TokenKind};
use regex::{Match, Regex};

pub fn tokenizer(content: String) -> Result<Vec<Token>, String> {
    let mut toks = vec![];

    let lines = content.lines();
    let rules = get_rules();

    let commentary: Regex = Regex::new(r"^--.*$").unwrap();
    let blank: Regex = Regex::new(r"^[ \t\r]*$").unwrap();

    for (line_idx, line) in lines.enumerate() {
        let line_idx = line_idx + 1;
        let mut line_content = line.to_string();

        let mut col = 1;
        'parser: loop {
            let line_content_len = line_content.len();
            line_content = line_content.trim().to_string();
            col += line_content_len - line_content.len();

            if blank.is_match(&line_content) {
                break 'parser;
            }

            if commentary.is_match(&line_content) {
                break 'parser;
            }

            for (tk, re) in rules.iter() {
                if let Some(mut tok) = parse_regex(re, &mut line_content, &tk) {
                    tok.set_line(line_idx);
                    tok.set_column(col);
                    col += tok.value_len();
                    toks.push(tok);
                    continue 'parser;
                }
            }

            return Err(format!(
                "Cannot parse {} at line {}",
                line_content,
                line_idx + 1
            ));
        }
    }

    Ok(toks)
}

fn get_rules() -> Vec<(TokenKind, Regex)> {
    let mut keyword_list = vec![
        "function", "end", "do", "task", "when", "record", "match", "if", "elif", "else", "while",
        "for", "in", "and", "or", "not", "xor", "return", "store", "break", "continue",
    ];
    let mut keyword_regex = format!("(({})", keyword_list.remove(0));
    for kw in keyword_list {
        keyword_regex.extend(format!("|({})", kw).chars());
    }
    keyword_regex.push(')');
    let keyword_regex = &format!("(^{0}[^_a-zA-Z0-9])|(^{0}$)", keyword_regex);
    vec![
        (
            TokenKind::StringLiteral,
            Regex::new("^((\".*?\")|('.*?'))").unwrap(),
        ),
        (
            TokenKind::IntegerLiteral,
            Regex::new(r"^-?(([1-9][0-9]*)|(0x[0-9a-fA-F]+)|(0x0+)|(0+))").unwrap(),
        ),
        (
            TokenKind::BooleanLiteral,
            Regex::new(r"^((true)|(false))").unwrap(),
        ),
        (TokenKind::NilLiteral, Regex::new(r"^nil").unwrap()),
        (TokenKind::Delimiter, Regex::new(r"^[()\[\],;]").unwrap()),
        (TokenKind::MatchDefaultOperator, Regex::new(r"^_").unwrap()),
        (TokenKind::RightArrow, Regex::new(r"^=>").unwrap()),
        (
            TokenKind::CompareOperator,
            Regex::new(r"^([=><!]=)|(><)").unwrap(),
        ),
        (
            TokenKind::AssignOperator,
            Regex::new(r"^[\+\-\*/%]?=").unwrap(),
        ),
        (
            TokenKind::ArithmeticOperator,
            Regex::new(r"^[\+\-\*/%]").unwrap(),
        ),
        (TokenKind::GuardOperator, Regex::new(r"^::").unwrap()),
        (TokenKind::TimeOperator, Regex::new(r"^@").unwrap()),
        (TokenKind::PipeOperator, Regex::new(r"^\.").unwrap()),
        (TokenKind::Keyword, Regex::new(keyword_regex).unwrap()),
        (
            TokenKind::Identifier,
            Regex::new(r"^[_a-zA-Z][_a-zA-Z0-9]*").unwrap(),
        ),
    ]
}

fn get_end_pos(line: &str, mat: &Match, token_kind: &TokenKind) -> usize {
    mat.end()
        - if let TokenKind::Keyword = token_kind {
            if line.chars().nth(mat.end() - 1).unwrap().is_alphabetic() {
                0
            } else {
                1
            }
        } else {
            0
        }
}

fn parse_regex(re: &Regex, line: &mut String, token_kind: &TokenKind) -> Option<Token> {
    let mut value = String::new();

    if let Some(mat) = re.find(line) {
        let end = get_end_pos(&line, &mat, &token_kind);

        value.extend(line.get(0..end));
        line.drain(0..end);

        Some(Token::new(token_kind.clone(), value))
    } else {
        None
    }
}
