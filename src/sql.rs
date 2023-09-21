use core::slice::SlicePattern;
use std::{array::TryFromSliceError, mem::size_of, num::ParseIntError};

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

impl Row {
    const ID_OFFSET: usize = 0;
    const ID_SIZE: usize = size_of_ret_type(|x: Self| x.id);
    const USERNAME_OFFSET: usize = Self::ID_OFFSET + Self::ID_SIZE;
    const USERNAME_SIZE: usize = size_of_ret_type(|x: Self| x.username);
    const EMAIL_OFFSET: usize = Self::USERNAME_OFFSET + Self::USERNAME_SIZE;
    const EMAIL_SIZE: usize = size_of_ret_type(|x: Self| x.email);

    pub fn copy_to_bytes(&self, dst: &mut [u8]) {
        let id_bytes = self.id.to_ne_bytes();
        assert_eq!(Self::ID_SIZE, id_bytes.len());
        dst[Self::ID_OFFSET..Self::ID_OFFSET + Self::ID_SIZE].copy_from_slice(id_bytes.as_slice());

        let username_start = Self::USERNAME_OFFSET;
        let username_end = username_start + Self::USERNAME_SIZE;
        assert_eq!(Self::USERNAME_SIZE, self.username.len());
        dst[username_start..username_end].copy_from_slice(self.username.as_slice());

        let email_start = Self::EMAIL_OFFSET;
        let email_end = email_start + Self::EMAIL_SIZE;
        assert_eq!(Self::EMAIL_SIZE, self.email.len());
        dst[email_start..email_end].copy_from_slice(self.email.as_slice());
    }

    pub fn from_bytes(&self, src: &[u8]) -> Row {
        let id_start = Self::ID_OFFSET;
        let id_end = id_start + Self::ID_SIZE;
        let id = u32::from_ne_bytes(src[id_start..id_end].try_into().unwrap());

        let username_start = Self::USERNAME_OFFSET;
        let username_end = username_start + Self::USERNAME_SIZE;
        let username = src[username_start..username_end].try_into().unwrap();

        let email_start = Self::EMAIL_OFFSET;
        let email_end = email_start + Self::EMAIL_SIZE;
        let email = src[email_start..email_end].try_into().unwrap();

        Row {
            id,
            username,
            email,
        }
    }
}

const fn size_of_ret_type<F, T, U>(_f: F) -> usize
where
    F: FnOnce(T) -> U,
{
    size_of::<U>()
}
