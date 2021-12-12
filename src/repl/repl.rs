use std::io::{self, Read, Write, BufRead, BufReader};

use crate::{
    ast,
    lexer::lexer::Lexer,
    parser::parser::Parser,
    interpreter::{evaluator, object::Environment},
    error::Error,
};

const PROMPT: &[u8; 4] = b">>> ";

pub fn start<I: Read, O: Write>(input: I, output: &mut O) -> Result<(), Error> {
    let mut bufio = BufReader::new(input);
    let mut buf = String::new();
    let mut env = Environment::new();

    loop {
        output.write_all(PROMPT)?;
        output.flush()?;
        bufio.read_line(&mut buf)?;

        let src = buf.bytes().map(|x| Ok(x)).peekable();
        let lex = &mut Lexer::new(src)?;
        let mut parser = Parser::new(lex.peekable())?;
        let program = parser.parse()?;

        if let Err(_) = print_parser_errors(output, parser.errors()) {
            continue;
        };

        let evalutated = evaluator::eval(ast::MNode::Prog(program), &mut env)?;

        output.write_all(format!("{}\n", evalutated).as_bytes())?;
        output.flush()?;
        buf.clear()
    }
}

fn print_parser_errors<O: Write>(output: &mut O, errors: Vec<String>) -> io::Result<()> {
    if errors.is_empty() {
        return Ok(())
    }

    output.write_all(b"Parser errors:\n")?;
    for e in errors {
        output.write_all(format!("\t{}\n", e).as_bytes())?;
    }

    output.flush()
}
