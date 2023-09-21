mod meta_command;
mod sql;
mod storage;

use meta_command::do_meta_command;
use sql::Row;
use storage::Table;

use std::io::Write;

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

        execute_statement(&mut table, &statement);
        println!("Executed!")
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

fn execute_statement(table: &mut Table, statement: &sql::Statement) {
    use sql::Statement as S;

    match statement {
        S::Insert { row_to_insert } => execute_insert(table, &row_to_insert),
        S::Select => execute_select(&table),
    }
}

fn execute_insert(table: &mut Table, row_to_insert: &Row) {
    table.insert_row(row_to_insert)
}

fn execute_select(table: &Table) {
    table.all_rows().for_each(|row| println!("{}", row))
}
