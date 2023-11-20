use std::io::Write;
use std::fs::{File, OpenOptions};
use std::path::PathBuf;
use super::csv::CSV;
use super::error::CSVError;

struct CSVWriter {
    path: PathBuf,
    data: CSV,
    file: File,
    initialized: bool,
    debug: bool
}

impl CSVWriter {
    pub fn new(path: Option<&str>, data: CSV) -> Result<Self, CSVError> {
        let path = Self::resolve_file_path(path);
        // if file exists, we assume it's already initialized (has at least headers)
        let initialized = path.exists();
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(&path)?;

        Ok(CSVWriter { path, data, file, initialized, debug: false })
    }
    
    pub fn enable_debug(&mut self)  {
        self.debug = true;
    }
    
    pub fn initialize(&mut self) -> Result<&mut Self, CSVError> {
        if self.initialized {
            return Err(CSVError::Other("File is already initialized".into()));
        }
    
        if self.data.headers.is_empty() {
            return Err(CSVError::NoHeaders);
        } 

        self.append(&self.data.build_header_string());
        
        Ok(self)
    }
    
    pub fn append(&mut self, row: &str) -> Result<(), CSVError> {
        writeln!(self.file, "{}", row)
            .map_err(|e| {
                eprintln!("Couldn't write to file: {}", e);
                CSVError::IOError(e)
            })
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
