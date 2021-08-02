use std::fs::File;
use std::io::prelude::*;

mod lexer;

fn main() -> std::io::Result<()> {
    let mut file = File::open("example.nx")?;
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)?;

    let toks = lexer::tokenizer(file_content);
    let toks = toks.unwrap();

    println!("Tokens: {:#?}", toks);

    Ok(())
}
