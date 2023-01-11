use std::fs::File;
use std::io::Read;
use std::{
    io::{self, Seek, SeekFrom},
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

pub const BUFFER_SIZE: usize = 256;
pub const page_size: u64 = 250;

pub enum move_values {
    up_line,
    down_line,
    up_page,
    down_page,
}
pub struct sedecim_file_info {
    pub buffer: [u8; BUFFER_SIZE],
    pub file_name: String,
    pub file_offset: u64,
    pub file_size: u64,
}

impl sedecim_file_info {
    pub fn new(file_name: String) -> sedecim_file_info {
        let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
        let mut file_offset: u64 = 0;
        let file_size: u64 = 0;
        sedecim_file_info {
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

    pub fn scroll(&mut self, scroll_amount: move_values) {
        match scroll_amount {
            move_values::up_line => {
                if self.file_offset >= 10 {
                    self.file_offset -= 10;
                    self.read_bytes();
                } else {
                    self.file_offset = 0;
                }
            }

            move_values::down_line => {
                if self.file_offset <= self.file_size - 10 {
                    self.file_offset += 10;
                    self.read_bytes();
                }
            }

            move_values::up_page => {
                if self.file_offset >= page_size {
                    self.file_offset -= page_size;
                    self.read_bytes();
                } else {
                    self.file_offset = 0;
                }
            }

            move_values::down_page => {
                if self.file_offset <= self.file_size - page_size {
                    self.file_offset += page_size;
                    self.read_bytes();
                }
            }
        };
    }
}
