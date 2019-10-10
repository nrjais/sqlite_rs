use rustyline::config::Configurer;
use rustyline::error::ReadlineError;
use sqlite::error::SqliteError;
use sqlite::row::Row;
use sqlite::statements::Statement::Insert;
use sqlite::statements::{InsertStatement, Statement};
use sqlite::table::Table;
use std::error::Error;
use std::process::exit;

fn main() {
  let mut table = Table::default();
  let mut rl = rustyline::Editor::<()>::new();
  rl.load_history("history.txt").unwrap_or(());
  rl.set_auto_add_history(true);
  rl.set_history_ignore_dups(true);
  rl.set_history_ignore_space(true);

  loop {
    let readline = rl.readline("sqlite>");
    match readline {
      Ok(line) => {
        repl_command(line, &mut table).unwrap_or(());
      }
      Err(ReadlineError::Eof) | Err(ReadlineError::Interrupted) => {
        exit(0);
      }
      Err(_) => {
        println!("No input");
      }
    };
    rl.save_history("history.txt").unwrap_or(());
  }
}

fn repl_command(c: String, table: &mut Table) -> Result<(), Box<dyn Error>> {
  if c.starts_with(".") {
    execute_meta_command(c.trim_start_matches("."));
  } else {
    execute_statement_command(c.as_str(), table)?;
  }

  Ok(())
}

fn execute_meta_command(c: &str) {
  match c {
    "exit" => exit(0),
    _ => println!("Unknown command {}", c),
  }
}

fn execute_statement_command(c: &str, table: &mut Table) -> Result<(), Box<dyn Error>> {
  let statement_result = prepare_statement(c);
  match statement_result {
    Ok(statement) => execute_statement(statement, table),
    Err(_) => Ok(println!("Unrecognized keyword at start of: {:?}", c)),
  }
}

fn prepare_statement(s: &str) -> Result<Statement, SqliteError> {
  match s {
    _ if s.starts_with("insert") => {
      let insert_statement = InsertStatement::parse(s)?;
      Ok(Insert(insert_statement))
    }
    _ if s.starts_with("select") => Ok(Statement::Select),
    _ => Err(SqliteError::UnknownStatementType),
  }
}

fn execute_statement(s: Statement, table: &mut Table) -> Result<(), Box<dyn Error>> {
  match s {
    Insert(InsertStatement { row }) => {
      let bytes = row.serialize()?;
      table.insert_row(bytes);
      println!("Inserted row: {}", row);
    }
    Statement::Select => {
      for bytes in table.rows() {
        let row = Row::deserialize(bytes)?;
        println!("{}", row);
      }
      println!("{} Rows", table.len());
    }
  };
  Ok(())
}
