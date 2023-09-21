mod meta_command;
mod sql;
mod storage;

use meta_command::do_meta_command;
use storage::{InsertError, Row, Table};
use thiserror::Error;

use std::io::{self, Write};

fn main() {
    let mut table = Table::new();
    loop {
        show_prompt();

        let buffer = read_input();

        if buffer.starts_with(".") {
            match do_meta_command(&buffer) {
                Ok(()) => continue,
                Err(e) => {
                    eprintln!("{}", e);
                    continue;
                }
            }
        }

        let statement = match sql::prepare_statement(&buffer) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
        };

        match execute_statement(&mut table, &statement) {
            Err(e) => {
                eprintln!("{}", e);
                continue;
            }
            Ok(_) => {}
        }
        println!("Executed!");
    }
}

fn show_prompt() {
    print!("db > ");
    std::io::stdout().flush().expect("Cannot flush stdout");
}

fn read_input() -> String {
    let mut buffer = String::new();

    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Error reading input!");

    buffer.trim().into()
}

#[derive(Debug, Error)]
enum ExecuteError {
    #[error("Unable to insert")]
    InsertError(#[from] InsertError),
    #[error("Unable to select")]
    IoError(#[from] io::Error),
}

fn execute_statement(table: &mut Table, statement: &sql::Statement) -> Result<(), ExecuteError> {
    use sql::Statement as S;

    match statement {
        S::Insert { row_to_insert } => execute_insert(table, &row_to_insert)?,
        S::Select => execute_select(&table)?,
    }

    Ok(())
}

fn execute_insert(table: &mut Table, row_to_insert: &Row) -> Result<(), InsertError> {
    table.insert_row(row_to_insert)
}

fn execute_select(table: &Table) -> io::Result<()> {
    for row in table.all_rows() {
        println!("{}", row?)
    }

    Ok(())
}
