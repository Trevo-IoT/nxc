use regex::Regex;

#[derive(Debug, Clone)]
pub enum TokenKind {
    Identifier,
    Delimiter,
    IntegerLiteral,
    ArithmeticOperator,
    AssingOperator,
    Keyword,
    StringLiteral,
    GuardOperator,
    TimeOperator,
    PipeOprator,
    RightArrow,
    MatchDefaultOperator,
    BooleanLiteral,
    NilLiteral,
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

fn parse_regex(re: &Regex, line: &mut String, token_kind: &TokenKind) -> Option<Token> {
    let mut value = String::new();

    if let Some(mat) = re.find(line) {
        let end = mat.end();

        for _i in 0..end {
            value.push(line.remove(0));
        }

        Some(Token::new(token_kind.clone(), value))
    } else {
        None
    }
}

pub fn tokenizer(content: String) -> Result<Vec<Token>, String> {
    let mut toks = vec![];

    let lines = content.lines();

    let mut keyword_list = vec![
        "function",
        "end",
        "do",
        "task",
        "when",
        "record",
        "match",
        "if",
        "elif",
        "else",
        "while",
        "for",
        "in"
    ];
    let mut keyword_regex = format!("(({})", keyword_list.remove(0));
    for kw in keyword_list {
        keyword_regex.extend(format!("|({})", kw).chars());
    }
    keyword_regex.push(')');
    let keyword_regex = &format!("(^{0}[^_a-zA-Z0-9])|(^{0}$)", keyword_regex);

    let commentary: Regex = Regex::new(r"^--.*$").unwrap();
    let blank: Regex = Regex::new(r"^[ \t\r]*$").unwrap();

    let rules: Vec<(TokenKind, Regex)> = vec![
        (TokenKind::StringLiteral, Regex::new("^((\".*?\")|('.*?'))").unwrap()),
        (TokenKind::IntegerLiteral, Regex::new(r"^-?(([1-9][0-9]*)|(0x[0-9a-fA-F]+)|(0x0+)|(0+))").unwrap()),
        (TokenKind::BooleanLiteral, Regex::new(r"^((true)|(false))").unwrap()),
        (TokenKind::NilLiteral, Regex::new(r"^nil").unwrap()),

        (TokenKind::Delimiter, Regex::new(r"^[()\[\],;]").unwrap()),

        (TokenKind::MatchDefaultOperator, Regex::new(r"^_").unwrap()),
        (TokenKind::RightArrow, Regex::new(r"^=>").unwrap()),
        (TokenKind::AssingOperator, Regex::new(r"^[\+\-\*/%]?=").unwrap()),
        (TokenKind::ArithmeticOperator, Regex::new(r"^[\+\-\*/%]").unwrap()),
        (TokenKind::GuardOperator, Regex::new(r"^::").unwrap()),
        (TokenKind::TimeOperator, Regex::new(r"^@").unwrap()),
        (TokenKind::PipeOprator, Regex::new(r"^\.").unwrap()),

        (TokenKind::Keyword, Regex::new(keyword_regex).unwrap()),
        (TokenKind::Identifier, Regex::new(r"^[_a-zA-Z][_a-zA-Z0-9]*").unwrap()),
    ];

    for (line_idx, line) in lines.enumerate() {
        let mut line_content = line.to_string();

        'parser: loop {
            line_content = line_content.trim().to_string();

            if blank.is_match(&line_content) {
                break 'parser;
            }

            if commentary.is_match(&line_content) {
                break 'parser;
            }

            for (tk, re) in rules.iter() {
                if let Some(tok) = parse_regex(re, &mut line_content, &tk) {
                    toks.push(tok);
                    continue 'parser;
                }
            }

            return Err(format!("Cannot parse {} at line {}", line_content, line_idx+1));
        }
    }

    Ok(toks)
}
