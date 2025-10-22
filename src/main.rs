use jsengine::parser::Parser;
use jsengine::interpreter::Interpreter;
use jsengine::value;
use std::io::{self, Write};
use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // Run file
        let filename = &args[1];
        match fs::read_to_string(filename) {
            Ok(source) => {
                if let Err(e) = run(&source) {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
            Err(e) => {
                eprintln!("Failed to read file '{}': {}", filename, e);
                std::process::exit(1);
            }
        }
    } else {
        // REPL mode
        repl();
    }
}

fn run(source: &str) -> Result<(), String> {
    let mut parser = Parser::new(source);
    let ast = parser.parse()?;

    let mut interpreter = Interpreter::new();
    interpreter.eval(&ast)?;

    Ok(())
}

fn repl() {
    println!("JSEngine v0.1.0 - JavaScript Engine in Rust");
    println!("Type JavaScript code and press Enter. Type '.exit' to quit.\n");

    let mut interpreter = Interpreter::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            break;
        }

        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        if input == ".exit" {
            println!("Goodbye!");
            break;
        }

        match Parser::new(input).parse() {
            Ok(ast) => {
                match interpreter.eval(&ast) {
                    Ok(value) => {
                        // Don't print undefined in REPL
                        if !matches!(value, value::Value::Undefined) {
                            println!("{}", value);
                        }
                    }
                    Err(e) => eprintln!("Runtime Error: {}", e),
                }
            }
            Err(e) => eprintln!("Parse Error: {}", e),
        }
    }
}
