use std::{io::BufWriter, fs::File};

use super::csv::CSV;

struct CSVWriter {
    name: String,
    data: CSV,
    exists: bool,
    buffer: Option<BufWriter<File>>
}

impl CSVWriter {
    pub fn new(name: Option<&str>) -> CSVWriter {
        CSVWriter {
            name: name.to_string(),
            exists: false,
        }
    }
    
    fn get_epoch_time(&self) -> u128 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
    }
}
