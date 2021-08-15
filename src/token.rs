use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Identifier,
    Delimiter,
    IntegerLiteral,
    ArithmeticOperator,
    CompareOperator,
    AssignOperator,
    Keyword,
    StringLiteral,
    GuardOperator,
    TimeOperator,
    PipeOperator,
    RightArrow,
    MatchDefaultOperator,
    BooleanLiteral,
    NilLiteral,
}

pub struct Token {
    kind: TokenKind,
    value: String,
    line: usize,
    column: usize,
}

impl Token {
    pub fn new(kind: TokenKind, value: String) -> Self {
        Self {
            kind,
            value,
            line: 0,
            column: 0,
        }
    }

    pub fn kind(&self) -> &TokenKind {
        &self.kind
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn value_len(&self) -> usize {
        self.value.len()
    }

    pub fn set_line(&mut self, line: usize) {
        self.line = line;
    }

    pub fn set_column(&mut self, column: usize) {
        self.column = column;
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}|{}: {:?},\"{}\")",
            self.line, self.column, self.kind, self.value
        )
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Token: <{:?},\"{}\",{}, {}>",
            self.kind, self.value, self.line, self.column
        )
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.value == other.value
    }
}
