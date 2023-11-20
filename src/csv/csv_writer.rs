use std::io::Write;
use std::fs::{File, OpenOptions};
use std::path::PathBuf;
use super::csv::CSV;
use super::error::CSVError;

struct CSVWriter {
    path: PathBuf,
    data: CSV,
    file: File,
}

impl CSVWriter {
    pub fn new(path: Option<&str>, data: CSV) -> Result<Self, CSVError> {
        let path = Self::resolve_file_path(path);
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(&path)?;

        Ok(CSVWriter { path, data, file })
    }
    
    pub fn append(&self, row: &str) -> Result<(), CSVError> {
        writeln!(self.file, "{}", row)?;
        Ok(())
    }

    fn resolve_file_path(path_str: Option<&str>) -> PathBuf {
        if let Some(path_str) = path_str {
            PathBuf::from(path_str)
        } else {
            let epoch_millis = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("Time went backwards")
                .as_millis();
            PathBuf::from(format!("{}.csv", epoch_millis))
        }
    }
    
    

}
