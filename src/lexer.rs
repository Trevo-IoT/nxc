#[derive(Debug)]
pub enum TokenKind {
    Identifier,
    Delimiter,
    IntegerLiteral,
    Operator,
}

#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    value: String,
}

impl Token {
    pub fn new(kind: TokenKind, value: String) -> Self {
        Self {kind, value}
    }
}

pub fn tokenizer(content: String) -> Result<Vec<Token>, String> {
    let mut toks = vec![];

    let lines = content.lines();

    let parsers = [
        parse_identifier,
        parse_delimiter,
        parse_integer_literal,
        parse_operator,
    ];

    for line in lines {
        let mut line_content = line.to_string();

        'parser: loop {
            line_content = line_content.trim().to_string();

            if is_blank_line(&line_content) {
                break 'parser;
            }

            if is_comment(&line_content) {
                break 'parser;
            }

            for parser in parsers.iter() {
                if let Some(tok) = parser(&mut line_content) {
                    toks.push(tok);
                    continue 'parser;
                }
            }

            return Err(format!("Line with invalid token: {}", line));
        }
    }

    Ok(toks)
}

fn is_blank_line(line: &str) -> bool {
    if !line.is_empty() {
        for ch in line.chars() {
            if ch != ' ' && ch != '\t' {
                return false;
            }
        }
    }

    true
}

fn is_comment(line: &str) -> bool {
    line.len() >= 2 && &line[0..2] == "--"
}

fn is_digit(ch: char) -> bool {
    '0' <= ch && ch <= '9'
}

fn is_digit_non_zero(ch: char) -> bool {
    '1' <= ch && ch <= '9'
}

fn is_lower_case(ch: char) -> bool {
    'a' <= ch && ch <= 'z'
}

fn is_upper_case(ch: char) -> bool {
    'A' <= ch && ch <= 'Z'
}

fn is_alphabet(ch: char) -> bool {
    is_lower_case(ch) || is_upper_case(ch)
}

fn is_alphanumeric(ch: char) -> bool {
    is_alphabet(ch) || is_digit(ch)
}

fn parse_identifier(content: &mut String) -> Option<Token> {
    let mut value = String::new();

    let ch = content.chars().nth(0).unwrap();
    if ch != '_' && !is_alphabet(ch) {
        return None;
    }
    value.push(content.remove(0));

    while !content.is_empty() {
        let ch = content.chars().nth(0).unwrap();
        if ch != '_' && !is_alphanumeric(ch) {
            break;
        }

        value.push(content.remove(0));
    }

    Some(Token::new(TokenKind::Identifier, value))
}

fn parse_delimiter(content: &mut String) -> Option<Token> {
    let ch = content.chars().nth(0).unwrap();

    if "[]();".to_string().contains(ch) {
        Some(Token::new(TokenKind::Delimiter, content.remove(0).to_string()))
    } else {
        None
    }
}

fn parse_integer_literal(content: &mut String) -> Option<Token> {
    let mut value = String::new();

    let ch = content.chars().nth(0).unwrap();
    if !is_digit_non_zero(ch) {
        return None;
    }
    value.push(content.remove(0));

    while !content.is_empty() {
        let ch = content.chars().nth(0).unwrap();
        if !is_digit(ch) {
            break;
        }

        value.push(content.remove(0));
    }

    Some(Token::new(TokenKind::IntegerLiteral, value))
}

fn parse_operator(content: &mut String) -> Option<Token> {
    let ch = content.chars().nth(0).unwrap();
    let mut value = String::new();

    if "+-*/%=".to_string().contains(ch) {
        value.push(content.remove(0));
        if ch != '=' && content.chars().nth(0).unwrap() == '=' {
            value.push(content.remove(0));
        }
        Some(Token::new(TokenKind::Operator, value))
    } else {
        None
    }
}
