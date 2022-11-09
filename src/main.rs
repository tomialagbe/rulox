use std::io;
use std::{
    env,
    fs::File,
    io::{BufRead, BufReader, Read, Write},
    process,
};

mod scanner;
mod token;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let mut rulox = RuLox::new();

    if args.len() > 2 {
        println!("Usage: rulox [script]");
        process::exit(64);
    } else if args.len() == 2 {
        let _r = rulox.run_file(&args[1]);
    } else {
        let _r = rulox.run_prompt();
    }
}

struct RuLox {
    had_error: bool,
}

impl RuLox {
    fn new() -> Self {
        RuLox { had_error: false }
    }

    fn run_file(&mut self, file_name: &str) -> Result<(), std::io::Error> {
        let Ok(file) = File::open(file_name) else {
            panic!("Cannot open {}", file_name);
        };

        let mut reader = BufReader::new(file);
        let mut buffer = String::new();
        reader.read_to_string(&mut buffer)?;
        self.run(&buffer);
        if self.had_error {
            process::exit(65);
        }
        Ok(())
    }

    fn run_prompt(&mut self) -> Result<(), io::Error> {
        let std_input = std::io::stdin();
        let mut reader = BufReader::new(std_input);
        let mut lock = std::io::stdout().lock();
        loop {
            print!("> ");
            io::stdout().flush()?;

            let mut line = String::new();
            let bytes_read = reader.read_line(&mut line)?;
            if bytes_read == 0 {
                break;
            }
            self.run(&line);

            // Reset error flag
            self.had_error = false;
            line.clear();
        }

        lock.flush()?;
        Ok(())
    }

    fn run(&mut self, source: &str) {
        use token::*;
        let scanner = scanner::Scanner::new(source);
        for token_res in scanner {
            match token_res {
                Ok(Token {
                    token_type: TokenType::Eof,
                    ..
                }) => {
                    break;
                }
                Ok(token) => {
                    println!("{:?}", token);
                }
                Err(err) => {
                    use scanner::ScanError::*;
                    match err {
                        UnexpectedCharacter { line, ch } => {
                            self.error(
                                line,
                                format!("Unexpected chatacter '{}' at line {}.", ch, line).as_str(),
                            );
                        }
                        UnterminatedString { line } => {
                            self.error(
                                line,
                                format!("Unterminated string at line {}.", line).as_str(),
                            );
                        }
                    }
                    break;
                }
            }
        }
    }

    fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }

    fn report(&mut self, line: usize, loc: &str, message: &str) {
        let mut err_out = io::stderr();
        let msg = format!("[line {line}] Error {loc}: {message}");

        let Ok(_) = err_out.write_all(msg.as_bytes()) else {
            panic!("Failed to write error to stderr");
        };
        self.had_error = true;
    }
}
