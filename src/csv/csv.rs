use super::error::CSVError;

#[derive(Debug)]
pub struct CSV {
    pub headers: Vec<String>,
    lines: Vec<String>,
    delimiter: char,
}

impl CSV {
    pub fn new() -> Self {
        Self {
            headers: vec![],
            lines: vec![],
            delimiter: ',',
        }
    }
    
    pub fn set_headers(&mut self, headers: Vec<String>) {
        self.headers = headers;
    }
    
    pub fn add_header(&mut self, header: String) {
        self.headers.push(header);
    }
    
    pub fn build_header_string(&self) -> String {
        self.headers.join(&self.delimiter.to_string())
    }
    
    pub fn add_line(&mut self, line: &str) {
        self.lines.push(line.to_string());
    }
    
    pub fn set_lines(&mut self, lines: Vec<String>) {
        self.lines = lines;
    }
    
    
    pub fn set_delimiter(&mut self, delimiter: char) -> Result<(), CSVError> {
        if delimiter.is_whitespace() || !delimiter.is_ascii(){
            return Err(CSVError::InvalidDelimiter(delimiter));
        }
        self.delimiter = delimiter;
        Ok(())
    }
}
