use std::array::TryFromSliceError;
use std::num::ParseIntError;

use thiserror::Error;

use crate::storage::Row;

pub enum Statement {
    Insert { row_to_insert: Row },
    Select,
}

#[derive(Debug, Error)]
pub enum PrepareError {
    #[error("Unrecognized keyword at start of '{0}'.")]
    UnrecognizedStatement(String),
    #[error("Not enough arguments")]
    NotEnoughArguments,
    #[error("Cannot parse number")]
    ParseError(#[from] ParseIntError),
    #[error("Argument too long")]
    ArgumentTooLong(#[from] TryFromSliceError),
}

pub fn prepare_statement(buffer: &String) -> Result<Statement, PrepareError> {
    if buffer.starts_with("insert") {
        let mut parts = buffer.split_ascii_whitespace();
        parts.next().unwrap();

        let id = parts
            .next()
            .ok_or(PrepareError::NotEnoughArguments)?
            .parse::<u32>()?;

        let username = parts.next().ok_or(PrepareError::NotEnoughArguments)?;

        let email = parts.next().ok_or(PrepareError::NotEnoughArguments)?;

        Ok(Statement::Insert {
            row_to_insert: Row::new(id, username, email),
        })
    } else if buffer == "select" {
        Ok(Statement::Select)
    } else {
        Err(PrepareError::UnrecognizedStatement(buffer.to_owned()))
    }
}
