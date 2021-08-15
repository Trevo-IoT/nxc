use crate::token::{Token, TokenKind};

#[derive(Debug)]
pub enum AST {
    Function {
        name: String,
        arguments: Vec<String>,
        body: Vec<Statement>,
    },
    Record {
        name: String,
        length: usize,
        data_size: usize,
    },
    Task {
        name: String,
        interval_ms: usize,
        body: Vec<Statement>,
    },
    When {
        interface: String,
        packet: String,
        guard: Guard,
        body: Vec<Statement>,
    },
}

#[derive(Debug)]
pub enum Guard {
    Numeric(isize),
    Regex(String),
}

#[derive(Debug)]
pub enum Statement {
    Assignment {
        variable: String,
        expression: Expression,
    },
    AssignmentSum {
        variable: String,
        expression: Expression,
    },
    AssignmentMinus {
        variable: String,
        expression: Expression,
    },
    AssignmentMult {
        variable: String,
        expression: Expression,
    },
    AssignmentDiv {
        variable: String,
        expression: Expression,
    },
    AssignmentMod {
        variable: String,
        expression: Expression,
    },
    Delay {
        time: usize,
    },
    Store {
        var_list: Vec<String>,
    },
    If {
        condition: Expression,
        body: Vec<Statement>,
        elif: Vec<(Expression, Vec<Statement>)>,
        else_body: Vec<Statement>,
    },
    For {
        var: String,
        collection: String,
        body: Vec<Statement>,
    },
    While {
        condition: Expression,
        body: Vec<Statement>,
    },
    Match {
        target: Expression,
        cases: Vec<(Literal, Vec<Statement>)>,
        default: Vec<Statement>,
    },
    Expression(Expression),
}

#[derive(Debug)]
pub enum Expression {
    Literal(Literal),
    Equal(Box<Expression>, Box<Expression>),
    NotEqual(Box<Expression>, Box<Expression>),
    Less(Box<Expression>, Box<Expression>),
    Greater(Box<Expression>, Box<Expression>),
    LessOrEqual(Box<Expression>, Box<Expression>),
    GreaterOrEqual(Box<Expression>, Box<Expression>),
    Sum(Box<Expression>, Box<Expression>),
    Minus(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Division(Box<Expression>, Box<Expression>),
    Modulus(Box<Expression>, Box<Expression>),
    Guard(Box<Expression>, Guard),
    Time(Box<Expression>, Box<Expression>),
    Pipe(Box<Expression>, FunctionCall),
    FunctionCall(FunctionCall),
}

#[derive(Debug)]
pub struct FunctionCall {
    name: String,
    arguments: Vec<Box<Expression>>,
}

#[derive(Debug)]
pub enum Literal {
    Integer(isize),
    String(String),
    Boolean(bool),
    Nil,
}

pub fn parse(mut tokens: Vec<Token>) -> Result<Vec<AST>, String> {
    let mut ast_list = vec![];

    while !tokens.is_empty() {
        if let Some(ast) = parse_function(&mut tokens)? {
            ast_list.push(ast);
            continue;
        }

        if let Some(ast) = parse_record(&mut tokens)? {
            ast_list.push(ast);
            continue;
        }

        if let Some(ast) = parse_task(&mut tokens)? {
            ast_list.push(ast);
            continue;
        }

        if let Some(ast) = parse_when(&mut tokens)? {
            ast_list.push(ast);
            continue;
        }
    }

    Ok(ast_list)
}

fn parse_function(tokens: &mut Vec<Token>) -> Result<Option<AST>, String> {
    if tokens.is_empty() {
        return Ok(None);
    }
    if tokens.get(0).unwrap() != &Token::new(TokenKind::Keyword, "function".to_string()) {
        return Ok(None);
    }
    tokens.remove(0);

    let name = tokens.get(0).ok_or("Not find function name")?;
    if name.kind() != &TokenKind::Identifier {
        return Err("The function 'keyword' must be followed by a identifier".to_string());
    }
    let name = name.value().to_string();
    tokens.remove(0);

    let arguments = parse_argument_name_list(tokens)?;

    let body = parse_block_statement(tokens)?;

    Ok(Some(AST::Function {
        name,
        arguments,
        body,
    }))
}

fn parse_record(tokens: &mut Vec<Token>) -> Result<Option<AST>, String> {
    if tokens.is_empty() {
        return Ok(None);
    }
    if tokens.get(0).unwrap() != &Token::new(TokenKind::Keyword, "record".to_string()) {
        return Ok(None);
    }
    tokens.remove(0);

    let name = tokens.get(0).ok_or("Not find function name")?;
    if name.kind() != &TokenKind::Identifier {
        return Err("The 'record' keyword must be followed by a identifier".to_string());
    }
    let name = name.value().to_string();
    tokens.remove(0);

    let (length, data_size) = parse_record_info(tokens)?;

    Ok(Some(AST::Record {
        name,
        length,
        data_size,
    }))
}

fn parse_task(tokens: &mut Vec<Token>) -> Result<Option<AST>, String> {
    if tokens.is_empty() {
        return Ok(None);
    }
    if tokens.get(0).unwrap() != &Token::new(TokenKind::Keyword, "task".to_string()) {
        return Ok(None);
    }
    tokens.remove(0);

    let name = tokens.get(0).ok_or("Not find task name")?;
    if name.kind() != &TokenKind::Identifier {
        return Err("The task 'keyword' must be followed by a identifier".to_string());
    }
    let name = name.value().to_string();
    tokens.remove(0);

    let interval_ms = parse_task_interval(tokens)?;

    let body = parse_block_statement(tokens)?;

    Ok(Some(AST::Task {
        name,
        interval_ms,
        body,
    }))
}

fn parse_when(tokens: &mut Vec<Token>) -> Result<Option<AST>, String> {
    if tokens.is_empty() {
        return Ok(None);
    }
    if tokens.get(0).unwrap() != &Token::new(TokenKind::Keyword, "when".to_string()) {
        return Ok(None);
    }
    tokens.remove(0);

    let interface = tokens.get(0).ok_or("Not find when interface".to_string())?;
    if interface.kind() != &TokenKind::StringLiteral {
        return Err("The 'when' keyword must be followed by a string literal".to_string());
    }
    let interface = interface.value().to_string();
    tokens.remove(0);

    let right_arrow = tokens.get(0).ok_or("Not find right arrow".to_string())?;
    if right_arrow.kind() != &TokenKind::RightArrow {
        return Err("Not find right arrow (=>) after 'when' interface".to_string());
    }
    tokens.remove(0);

    let packet = tokens.get(0).ok_or("Not find when packet".to_string())?;
    if packet.kind() != &TokenKind::Identifier {
        return Err("Not find 'when' packet identifier after right arrow".to_string());
    }
    let packet = packet.value().to_string();
    tokens.remove(0);

    let guard_symbol = tokens.get(0).ok_or("Not find guard operator".to_string())?;
    if guard_symbol.kind() != &TokenKind::GuardOperator {
        return Err("Not find :: after 'when' packet name".to_string());
    }
    tokens.remove(0);

    let guard = tokens.get(0).ok_or("Not find guard".to_string())?;
    let guard = if guard.kind() != &TokenKind::IntegerLiteral {
        Guard::Numeric(guard.value().parse::<isize>().unwrap())
    } else if guard.kind() != &TokenKind::StringLiteral {
        Guard::Regex(guard.value().to_string())
    } else {
        return Err(
            "The next token after :: must be a integer literal or a string literal".to_string(),
        );
    };
    tokens.remove(0);

    let body = parse_block_statement(tokens)?;

    Ok(Some(AST::When {
        interface,
        packet,
        guard,
        body,
    }))
}

fn parse_argument_name_list(tokens: &mut Vec<Token>) -> Result<Vec<String>, String> {
    let open_bracket = tokens.get(0).ok_or("Missing a open bracket")?;
    if open_bracket != &Token::new(TokenKind::Delimiter, "(".to_string()) {
        return Err("Missing a open bracket".to_string());
    }
    tokens.remove(0);

    todo!()
}

fn parse_block_statement(tokens: &mut Vec<Token>) -> Result<Vec<Statement>, String> {
    todo!()
}

fn parse_record_info(tokens: &mut Vec<Token>) -> Result<(usize, usize), String> {
    let open_brace = tokens.get(0).ok_or("Missing a open brace")?;
    if open_brace != &Token::new(TokenKind::Delimiter, "[".to_string()) {
        return Err("Missing a open brace".to_string());
    }
    tokens.remove(0);

    let length = tokens.get(0).ok_or("Missing record length".to_string())?;
    if length.kind() != &TokenKind::IntegerLiteral {
        return Err("Missing record length".to_string());
    }
    let length = tokens.remove(0).value().parse::<isize>().unwrap();
    if length < 0 {
        return Err("The record length cannot be negative".to_string());
    }

    let next_tok = tokens.get(0).ok_or("Missing close brace".to_string())?;
    let data_size = if next_tok == &Token::new(TokenKind::Delimiter, ",".to_string()) {
        let data_size = tokens
            .get(0)
            .ok_or("Missing record data size".to_string())?;
        if data_size.kind() != &TokenKind::IntegerLiteral {
            return Err("Missing record data size".to_string());
        }
        let data_size = tokens.remove(0).value().parse::<isize>().unwrap();
        if data_size < 0 {
            return Err("The record data size cannot be negative".to_string());
        }
        data_size
    } else if next_tok == &Token::new(TokenKind::Delimiter, "]".to_string()) {
        tokens.remove(0);
        1
    } else {
        return Err("Missing a close brace".to_string());
    };

    let semicolon = tokens
        .get(0)
        .ok_or("Missing ; at end of record statement")?;
    if semicolon != &Token::new(TokenKind::Delimiter, ";".to_string()) {
        return Err("Missing ; at end of record statement".to_string());
    }
    tokens.remove(0);

    Ok((length as usize, data_size as usize))
}

fn parse_task_interval(tokens: &mut Vec<Token>) -> Result<usize, String> {
    todo!()
}
