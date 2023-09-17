use thiserror::Error;

pub enum Statement {
    Insert,
    Select,
}

#[derive(Debug, Error)]
#[error("Unrecognized keyword at start of '{0}'.")]
pub struct UnrecognizedStatement(String);

pub fn prepare_statement(buffer: &String) -> Result<Statement, UnrecognizedStatement> {
    if buffer.starts_with("insert") {
        Ok(Statement::Insert)
    } else if buffer == "select" {
        Ok(Statement::Select)
    } else {
        Err(UnrecognizedStatement(buffer.to_owned()))
    }
}
