use std::fs::File;
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
        let file = File::create(&path)?;

        Ok(CSVWriter { path, data, file })
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
