use std::process::exit;
use rustyline::error::ReadlineError;

fn main() {
  let mut rl = rustyline::Editor::<()>::new();
  loop {
    let readline = rl.readline("sqlite>");
    match readline {
      Ok(line) => repl_command(line),
      Err(ReadlineError::Eof) | Err(ReadlineError::Interrupted) => exit(0),
      Err(_) => println!("No input"),
    }
  }
}

fn repl_command(c: String) {
  if c.starts_with(".") {
    execute_meta_command(c.trim_start_matches("."))
  } else {
    execute_statement_command(c.as_str())
  }
}

fn execute_meta_command(c: &str) {
  match c {
    "exit" => exit(0),
    _ => println!("Unknown command {}", c)
  }
}

struct Statement;

fn execute_statement_command(c: &str) {
  let statement = prepare_statement(c);
  execute_statement(statement);
}

fn prepare_statement(s: &str) -> Statement {
  Statement
}

fn execute_statement(s: Statement) {
  println!("Executed statement");
}