use std::{array::TryFromSliceError, num::ParseIntError};

use thiserror::Error;

struct Row {
    id: u32,
    username: [u8; 32],
    email: [u8; 256],
}

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

        let username = parts
            .next()
            .ok_or(PrepareError::NotEnoughArguments)?
            .as_bytes();

        let email = parts
            .next()
            .ok_or(PrepareError::NotEnoughArguments)?
            .as_bytes();

        Ok(Statement::Insert {
            row_to_insert: Row {
                id,
                username: username.try_into()?,
                email: email.try_into()?,
            },
        })
    } else if buffer == "select" {
        Ok(Statement::Select)
    } else {
        Err(PrepareError::UnrecognizedStatement(buffer.to_owned()))
    }
}
