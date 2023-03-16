use super::sedecim_file_info::{BUFFER_SIZE, BUFFER_SIZE_U64};

pub struct SedecimFilePage {
    pub page_id: u64,
    pub page_start: u64,
    pub loaded: bool,
    pub buffer: [u8; BUFFER_SIZE],
}

impl SedecimFilePage {
    pub fn new() -> SedecimFilePage {
        let page_id = 0;
        let page_start: u64 = 0;
        let loaded = false;
        let buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
        Self {
            page_id,
            page_start,
            loaded,
            buffer,
        }
    }

    pub fn get_page(address: u64) -> u64 {
        address / BUFFER_SIZE_U64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_page_zero() {
        let page = SedecimFilePage::get_page(0);

        assert_eq!(page, 0);
    }

    #[test]
    fn get_page_one() {
        let page = SedecimFilePage::get_page(256);

        assert_eq!(page, 1);
    }

    #[test]
    fn get_page_ten() {
        let page = SedecimFilePage::get_page(2560);

        assert_eq!(page, 10);
    }
}
