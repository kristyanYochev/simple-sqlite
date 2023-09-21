use std::array::TryFromSliceError;
use std::io::{self, Read, Write};
use std::mem::size_of;
use std::num::ParseIntError;

use thiserror::Error;

const USERNAME_LENGTH: usize = 32;
const EMAIL_LENGTH: usize = 256;
struct Row {
    id: u32,
    username: [u8; USERNAME_LENGTH],
    email: [u8; EMAIL_LENGTH],
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
    pub fn write_to_buffer(&self, dst: &mut dyn Write) -> io::Result<()> {
        let id_bytes = self.id.to_ne_bytes();
        dst.write(id_bytes.as_slice())?;
        dst.write(self.username.as_slice())?;
        dst.write(self.email.as_slice())?;
        Ok(())
    }

    pub fn from_buffer(src: &mut dyn Read) -> io::Result<Row> {
        let mut id_bytes = [0u8; size_of::<u32>()];
        src.read_exact(&mut id_bytes)?;
        let id = u32::from_ne_bytes(id_bytes);

        let mut username = [0u8; USERNAME_LENGTH];
        src.read_exact(&mut username)?;

        let mut email = [0u8; EMAIL_LENGTH];
        src.read_exact(&mut email)?;

        Ok(Row {
            id,
            username,
            email,
        })
    }

    pub fn new(id: u32, username: &str, email: &str) -> Row {
        let mut username_buffer = [0u8; USERNAME_LENGTH];
        let mut email_buffer = [0u8; EMAIL_LENGTH];

        username_buffer
            .as_mut_slice()
            .write(username.as_bytes())
            .unwrap();
        email_buffer.as_mut_slice().write(email.as_bytes()).unwrap();

        Row {
            id,
            username: username_buffer,
            email: email_buffer,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Row;

    #[test]
    fn serialization_and_deserialization_are_reverses_of_eachother() {
        let mut test_memory = [0u8; 512];

        let row = Row::new(3, "Test User", "test@tester.com");

        row.write_to_buffer(&mut test_memory.as_mut_slice())
            .unwrap();

        let deserialized_row = Row::from_buffer(&mut test_memory.as_slice()).unwrap();

        assert_eq!(row.id, deserialized_row.id);
        assert_eq!(row.username, deserialized_row.username);
        assert_eq!(row.email, deserialized_row.email);
    }
}
