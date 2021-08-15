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
    Return {
        expression: Expression,
    },
    FunctionCall(FunctionCall),
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

        return Err("Statement isn't valid".to_string());
    }

    Ok(ast_list)
}

macro_rules! check_first_keyword {
    ($tokens: expr, $kw: expr) => {
        if $tokens.is_empty() {
            return Ok(None);
        }
        if $tokens.get(0).unwrap() != &Token::new(TokenKind::Keyword, $kw.to_string()) {
            return Ok(None);
        }
        $tokens.remove(0);
    };
}

macro_rules! retrieve_tokenkind {
    ($tokens: expr, $tk_kind: expr, $err_msg: expr) => {{
        let tk = $tokens.get(0).ok_or($err_msg.to_string())?;
        if tk.kind() != &$tk_kind {
            return Err($err_msg.to_string());
        }
        $tokens.remove(0).value()
    }};
}

macro_rules! retrieve_tokenkind_or_none {
    ($tokens: expr, $tk_kind: expr, $err_msg: expr) => {{
        let tk = $tokens.get(0).ok_or($err_msg.to_string())?;
        if tk.kind() != &$tk_kind {
            None
        } else {
            Some($tokens.remove(0))
        }
    }};
}

macro_rules! retrieve_token {
    ($tokens: expr, $tk: expr, $err_msg: expr) => {{
        let tk = $tokens.get(0).ok_or($err_msg.to_string())?;
        if tk != &$tk {
            return Err($err_msg.to_string());
        }
        $tokens.remove(0).value()
    }};
}

macro_rules! retrieve_token_or_none {
    ($tokens: expr, $tk: expr, $err_msg: expr) => {{
        let tk = $tokens.get(0).ok_or($err_msg.to_string())?;
        if tk != &$tk {
            None
        } else {
            Some($tokens.remove(0))
        }
    }};
}

fn parse_function(tokens: &mut Vec<Token>) -> Result<Option<AST>, String> {
    check_first_keyword!(tokens, "function");

    let name = retrieve_tokenkind!(
        tokens,
        TokenKind::Identifier,
        "The 'function' keyword requires an identifier"
    )
    .to_string();

    let arguments = parse_argument_name_list(tokens)?;

    let body = parse_block_statement(tokens)?;

    Ok(Some(AST::Function {
        name,
        arguments,
        body,
    }))
}

fn parse_record(tokens: &mut Vec<Token>) -> Result<Option<AST>, String> {
    check_first_keyword!(tokens, "record");

    let name = retrieve_tokenkind!(
        tokens,
        TokenKind::Identifier,
        "The 'record' keyword requires an identifier"
    )
    .to_string();

    let (length, data_size) = parse_record_info(tokens)?;

    Ok(Some(AST::Record {
        name,
        length,
        data_size,
    }))
}

fn parse_task(tokens: &mut Vec<Token>) -> Result<Option<AST>, String> {
    check_first_keyword!(tokens, "task");

    let name = retrieve_tokenkind!(
        tokens,
        TokenKind::Identifier,
        "The 'task' keyword requires an identifier"
    )
    .to_string();

    let interval_ms = parse_task_interval(tokens)?;

    let body = parse_block_statement(tokens)?;

    Ok(Some(AST::Task {
        name,
        interval_ms,
        body,
    }))
}

fn parse_when(tokens: &mut Vec<Token>) -> Result<Option<AST>, String> {
    check_first_keyword!(tokens, "when");

    let interface = retrieve_tokenkind!(
        tokens,
        TokenKind::StringLiteral,
        "The 'when' keyword requires a string literal"
    )
    .to_string();

    retrieve_tokenkind!(
        tokens,
        TokenKind::RightArrow,
        "Not found right arrow (=>) after 'when' interface"
    );

    let packet = retrieve_tokenkind!(
        tokens,
        TokenKind::Identifier,
        "Not found 'when' packet variable after right arrow (=>)"
    )
    .to_string();

    retrieve_tokenkind!(
        tokens,
        TokenKind::GuardOperator,
        "Not found :: after 'when' packet variable"
    );

    let guard = if let Some(numeric_guard) =
        retrieve_tokenkind_or_none!(tokens, TokenKind::IntegerLiteral, "Not find guard")
    {
        Guard::Numeric(numeric_guard.value().parse::<isize>().unwrap())
    } else if let Some(regex_guard) =
        retrieve_tokenkind_or_none!(tokens, TokenKind::StringLiteral, "Not find guard")
    {
        Guard::Regex(regex_guard.value().to_string())
    } else {
        return Err(
            "The next token after :: must be a integer literal or a string literal".to_string(),
        );
    };

    let body = parse_block_statement(tokens)?;

    Ok(Some(AST::When {
        interface,
        packet,
        guard,
        body,
    }))
}

fn parse_argument_name_list(tokens: &mut Vec<Token>) -> Result<Vec<String>, String> {
    let mut arg_name_list = vec![];

    retrieve_token!(
        tokens,
        Token::new(TokenKind::Delimiter, "(".to_string()),
        "Missing an open bracket"
    );

    if let Some(first_arg) =
        retrieve_tokenkind_or_none!(tokens, TokenKind::Identifier, "Not find first argument")
    {
        arg_name_list.push(first_arg.value().to_string());

        loop {
            if let Some(_) = retrieve_token_or_none!(
                tokens,
                Token::new(TokenKind::Delimiter, ")".to_string()),
                "Missing a close bracket"
            ) {
                break;
            }

            retrieve_token!(
                tokens,
                Token::new(TokenKind::Delimiter, ",".to_string()),
                "Missing comma after argument name"
            );

            arg_name_list.push(
                retrieve_tokenkind!(
                    tokens,
                    TokenKind::Identifier,
                    "The arguments in function definition must be a identifier"
                )
                .to_string(),
            );
        }
    } else {
        retrieve_token!(
            tokens,
            Token::new(TokenKind::Delimiter, ")".to_string()),
            "Missing a close bracket"
        );
    }

    Ok(arg_name_list)
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
    if let Some(_) =
        retrieve_tokenkind_or_none!(tokens, TokenKind::TimeOperator, "Missing time operator")
    {
        let interval_ms = retrieve_tokenkind!(
            tokens,
            TokenKind::IntegerLiteral,
            "Not found task interval after @"
        )
        .parse::<isize>()
        .unwrap();
        if interval_ms < 0 {
            return Err("The task interval cannot be a negative integer".to_string());
        }

        Ok(interval_ms as usize)
    } else {
        Ok(0)
    }
}

fn parse_block_statement(tokens: &mut Vec<Token>) -> Result<Vec<Statement>, String> {
    let mut statements = vec![];

    loop {
        if let Some(_) = retrieve_token_or_none!(
            tokens,
            Token::new(TokenKind::Keyword, "end".to_string()),
            "Not found 'end' keyword"
        ) {
            break;
        }

        statements.push(parse_statement(tokens)?);
    }

    Ok(statements)
}

fn parse_statement(tokens: &mut Vec<Token>) -> Result<Statement, String> {
    if let Some(statement) = parse_assignment(tokens)? {
        return Ok(statement);
    }

    if let Some(statement) = parse_assignment_sum(tokens)? {
        return Ok(statement);
    }

    if let Some(statement) = parse_assignment_minus(tokens)? {
        return Ok(statement);
    }

    if let Some(statement) = parse_assignment_mult(tokens)? {
        return Ok(statement);
    }

    if let Some(statement) = parse_assignment_div(tokens)? {
        return Ok(statement);
    }

    if let Some(statement) = parse_assignment_mod(tokens)? {
        return Ok(statement);
    }

    if let Some(statement) = parse_delay(tokens)? {
        return Ok(statement);
    }

    if let Some(statement) = parse_store(tokens)? {
        return Ok(statement);
    }

    if let Some(statement) = parse_if(tokens)? {
        return Ok(statement);
    }

    if let Some(statement) = parse_for(tokens)? {
        return Ok(statement);
    }

    if let Some(statement) = parse_while(tokens)? {
        return Ok(statement);
    }

    if let Some(statement) = parse_match(tokens)? {
        return Ok(statement);
    }

    if let Some(statement) = parse_return(tokens)? {
        return Ok(statement);
    }

    if let Some(statement) = parse_statement_function_call(tokens)? {
        return Ok(statement);
    }

    return Err("Statement isn't valid".to_string());
}

fn parse_assignment(tokens: &mut Vec<Token>) -> Result<Option<Statement>, String> {
    todo!()
}

fn parse_assignment_sum(tokens: &mut Vec<Token>) -> Result<Option<Statement>, String> {
    todo!()
}

fn parse_assignment_minus(tokens: &mut Vec<Token>) -> Result<Option<Statement>, String> {
    todo!()
}

fn parse_assignment_mult(tokens: &mut Vec<Token>) -> Result<Option<Statement>, String> {
    todo!()
}

fn parse_assignment_div(tokens: &mut Vec<Token>) -> Result<Option<Statement>, String> {
    todo!()
}

fn parse_assignment_mod(tokens: &mut Vec<Token>) -> Result<Option<Statement>, String> {
    todo!()
}

fn parse_delay(tokens: &mut Vec<Token>) -> Result<Option<Statement>, String> {
    todo!()
}

fn parse_store(tokens: &mut Vec<Token>) -> Result<Option<Statement>, String> {
    todo!()
}

fn parse_if(tokens: &mut Vec<Token>) -> Result<Option<Statement>, String> {
    todo!()
}

fn parse_for(tokens: &mut Vec<Token>) -> Result<Option<Statement>, String> {
    todo!()
}

fn parse_while(tokens: &mut Vec<Token>) -> Result<Option<Statement>, String> {
    todo!()
}

fn parse_match(tokens: &mut Vec<Token>) -> Result<Option<Statement>, String> {
    todo!()
}

fn parse_return(tokens: &mut Vec<Token>) -> Result<Option<Statement>, String> {
    todo!()
}

fn parse_statement_function_call(tokens: &mut Vec<Token>) -> Result<Option<Statement>, String> {
    todo!()
}
