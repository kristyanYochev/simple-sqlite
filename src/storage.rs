use crate::sql::{Row, ROW_SIZE};

const MAX_PAGES: usize = 100;
pub struct Table {
    num_rows: usize,
    pages: [Option<Box<Page>>; MAX_PAGES],
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
        Row::from_buffer(&mut self.row_slot(row_num)).unwrap()
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
