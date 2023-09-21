use std::fmt;
use std::io::{self, Read, Write};
use std::mem::size_of;

const MAX_PAGES: usize = 100;
pub struct Table {
    num_rows: usize,
    pages: [Option<Box<Page>>; MAX_PAGES],
}

const USERNAME_LENGTH: usize = 32;
const EMAIL_LENGTH: usize = 256;
pub const ROW_SIZE: usize =
    size_of::<u32>() + size_of::<[u8; USERNAME_LENGTH]>() + size_of::<[u8; EMAIL_LENGTH]>();
pub struct Row {
    id: u32,
    username: [u8; USERNAME_LENGTH],
    email: [u8; EMAIL_LENGTH],
}

const PAGE_SIZE: usize = 4096;
const ROWS_PER_PAGE: usize = PAGE_SIZE / ROW_SIZE;

struct Page {
    memory: [u8; PAGE_SIZE],
}

impl Table {
    pub const MAX_ROWS: usize = ROWS_PER_PAGE * MAX_PAGES;

    pub fn new() -> Self {
        Self {
            num_rows: 0,
            pages: std::array::from_fn(|_| None),
        }
    }

    fn row_slot_mut(&mut self, row_num: usize) -> &mut [u8] {
        let page_num = row_num / ROWS_PER_PAGE;
        let page = self.pages.get_mut(page_num).unwrap();
        if page.is_none() {
            *page = Some(Box::default());
        }
        let page = page.as_mut().unwrap();

        let row_offset = row_num % ROWS_PER_PAGE;
        let byte_offset = row_offset * ROW_SIZE;

        &mut page.memory[byte_offset..]
    }

    fn row_slot(&self, row_num: usize) -> &[u8] {
        let page_num = row_num / ROWS_PER_PAGE;
        let page = self.pages.get(page_num).unwrap();
        let page = page.as_ref().unwrap();

        let row_offset = row_num % ROWS_PER_PAGE;
        let byte_offset = row_offset * ROW_SIZE;

        &page.memory[byte_offset..]
    }

    pub fn insert_row(&mut self, row: &Row) {
        if self.num_rows >= Self::MAX_ROWS {
            panic!("Not enough space!");
            todo!("Create failure type");
        }

        row.write_to_buffer(&mut self.row_slot_mut(self.num_rows))
            .unwrap();

        self.num_rows += 1;
    }

    pub fn all_rows(&self) -> impl Iterator<Item = Row> + '_ {
        (0..self.num_rows).map(|row_num| self.get_row(row_num))
    }

    fn get_row(&self, row_num: usize) -> Row {
        Row::read_from_buffer(&mut self.row_slot(row_num)).unwrap()
    }
}

impl Page {
    fn new() -> Self {
        Self {
            memory: [0; PAGE_SIZE],
        }
    }
}

impl Default for Page {
    fn default() -> Self {
        Page::new()
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

    pub fn read_from_buffer(src: &mut dyn Read) -> io::Result<Row> {
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

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.id,
            String::from_utf8_lossy(&self.username),
            String::from_utf8_lossy(&self.email)
        )
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

        let deserialized_row = Row::read_from_buffer(&mut test_memory.as_slice()).unwrap();

        assert_eq!(row.id, deserialized_row.id);
        assert_eq!(row.username, deserialized_row.username);
        assert_eq!(row.email, deserialized_row.email);
    }
}
