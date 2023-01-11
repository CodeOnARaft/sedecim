use std::fs::File;
use std::io::Read;
use std::{
    io::{ Seek, SeekFrom},
};

pub const BUFFER_SIZE: usize = 256;
pub const PAGE_SIZE: u64 = 250;

pub enum MoveValues {
    UpLine,
    DownLine,
    UpPage,
    DownPage,
}
pub struct SedecimFileInfo {
    pub buffer: [u8; BUFFER_SIZE],
    pub file_name: String,
    pub file_offset: u64,
    pub file_size: u64,
}

impl SedecimFileInfo {
    pub fn new(file_name: String) -> SedecimFileInfo {
        let buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
        let file_offset: u64 = 0;
        let file_size: u64 = 0;
        SedecimFileInfo {
            buffer,
            file_name,
            file_offset,
            file_size,
        }
    }

    pub fn read_bytes(&mut self) {
        let mut file = File::open(&self.file_name).unwrap();

        self.file_size = std::fs::metadata(&self.file_name).unwrap().len();
        let read = file.by_ref();
        let _ = read.seek(SeekFrom::Start(self.file_offset)).unwrap();
        let _ = read.take(256).read(&mut self.buffer).unwrap();
    }

    pub fn scroll(&mut self, scroll_amount: MoveValues) {
        match scroll_amount {
            MoveValues::UpLine => {
                if self.file_offset >= 10 {
                    self.file_offset -= 10;
                    self.read_bytes();
                } else {
                    self.file_offset = 0;
                }
            }

            MoveValues::DownLine => {
                if self.file_offset <= self.file_size - 10 {
                    self.file_offset += 10;
                    self.read_bytes();
                }
            }

            MoveValues::UpPage => {
                if self.file_offset >= PAGE_SIZE {
                    self.file_offset -= PAGE_SIZE;
                    self.read_bytes();
                } else {
                    self.file_offset = 0;
                }
            }

            MoveValues::DownPage => {
                if self.file_offset <= self.file_size - PAGE_SIZE {
                    self.file_offset += PAGE_SIZE;
                    self.read_bytes();
                }
            }
        };
    }
}
