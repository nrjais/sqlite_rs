use std::process::exit;
use rustyline::error::ReadlineError;
use crate::Statement::Insert;
use scan_fmt::scan_fmt;
use scan_fmt::parse::ScanError;

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

#[derive(Debug)]
struct Row {
  id: usize,
  username: String,
  email: String,
}

#[derive(Debug)]
struct InsertStatement {
  row: Row
}

impl<'a> InsertStatement {
  fn parse(args: &'a str) -> Result<InsertStatement, ParseError> {
    let row = Self::parse_row(args)?;
    Ok(InsertStatement { row })
  }

  fn parse_row(args: &str) -> Result<Row, ParseError> {
    let (id, username, email) = scan_fmt!(args, "insert {} {} {}", usize, String, String)?;
    Ok(Row { id, username, email })
  }
}

#[derive(Debug)]
enum Statement {
  Insert(InsertStatement),
  Select,
}

#[derive(Debug)]
enum ParseError {
  UnknownStatementType,
  UnknownParserError(String),
}

impl From<ScanError> for ParseError {
  fn from(e: ScanError) -> Self {
    ParseError::UnknownParserError(e.0)
  }
}

fn execute_statement_command(c: &str) {
  let statement_result = prepare_statement(c);
  match statement_result {
    Ok(statement) => execute_statement(statement),
    Err(_) => println!("Unrecognized keyword at start of: {:?}", c),
  }
}

fn prepare_statement(s: &str) -> Result<Statement, ParseError> {
  match s {
    _ if s.starts_with("insert") => {
      let insert_statement = InsertStatement::parse(s)?;
      Ok(Insert(insert_statement))
    }
    _ if s.starts_with("select") => Ok(Statement::Select),
    _ => Err(ParseError::UnknownStatementType)
  }
}

fn execute_statement(s: Statement) {
  println!("Executed statement: {:?}", s);
}