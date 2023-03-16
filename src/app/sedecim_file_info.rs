use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::io::{Seek, SeekFrom};
use std::rc::Rc;

use super::sedecim_file_page::SedecimFilePage;

pub const BUFFER_SIZE: usize = 250;
pub const BUFFER_SIZE_U64: u64 = 250;
pub const LINE_SIZE: u64 = 10;

pub enum MoveValues {
    UpLine,
    DownLine,
    UpPage,
    DownPage,
}
pub struct SedecimFileInfo {
    pub file_name: String,
    pub file_offset: u64,
    pub file_size: u64,
    pages: HashMap<u64, Rc<SedecimFilePage>>,
}

impl SedecimFileInfo {
    pub fn new(file_name: String) -> SedecimFileInfo {
        let file_offset: u64 = 0;
        let file_size: u64 = 0;
        let pages = HashMap::new();

        SedecimFileInfo {
            file_name,
            file_offset,
            file_size,
            pages,
        }
    }

    pub fn set_address(&mut self, address: u64) {
        self.file_offset = address;
        self.read_bytes(address);
    }

    fn read_bytes(&mut self, load_address: u64) {
        let current_page_number = SedecimFilePage::get_page(load_address);

        if !self.pages.contains_key(&current_page_number) {
            let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
            let mut file = File::open(&self.file_name).unwrap();

            self.file_size = std::fs::metadata(&self.file_name).unwrap().len();
            let read = file.by_ref();
            let _ = read.seek(SeekFrom::Start(self.file_offset)).unwrap();
            let _ = read.take(BUFFER_SIZE_U64).read(&mut buffer).unwrap();

            let mut page = SedecimFilePage::new();
            page.loaded = true;
            page.page_id = current_page_number;
            page.buffer = buffer;
            page.page_start = load_address - (load_address % BUFFER_SIZE_U64);

            self.pages.insert(current_page_number, Rc::new(page));
        }
    }

    pub fn get_page(&mut self, address: u64) -> Rc<SedecimFilePage> {
        self.read_bytes(address);

        match self.pages.get(&SedecimFilePage::get_page(address)) {
            Some(page) => return page.clone(),
            None => panic!("Page failed to Load."),
        }
    }

    pub fn scroll(&mut self, scroll_amount: MoveValues) {
        match scroll_amount {
            MoveValues::UpLine => {
                if self.file_offset >= LINE_SIZE {
                    self.set_address(self.file_offset - LINE_SIZE);
                } else {
                    self.set_address(0);
                }
            }

            MoveValues::DownLine => {
                if self.file_offset <= self.file_size - LINE_SIZE {
                    self.set_address(self.file_offset + LINE_SIZE);
                }
            }

            MoveValues::UpPage => {
                if self.file_offset >= BUFFER_SIZE_U64 {
                    self.set_address(self.file_offset - BUFFER_SIZE_U64);
                } else {
                    self.set_address(0);
                }
            }

            MoveValues::DownPage => {
                if self.file_offset <= self.file_size - BUFFER_SIZE_U64 {
                    self.set_address(self.file_offset + BUFFER_SIZE_U64);
                }
            }
        };
    }
}
