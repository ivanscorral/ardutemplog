use std::{io::{self, Read}, time::Duration, thread};
use std::fs::OpenOptions;
use std::io::Write;

mod csv;

#[derive(Debug, Clone, Copy)]
struct Measurement {
    temperature: f32,
    humidity: f32,
    incremental_timestamp: u128,
}

impl Measurement {
    fn new(temperature: f32, humidity: f32, initial_timestamp: u128) -> Self {
        let incremental_timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() - initial_timestamp;
       Self { temperature, humidity, incremental_timestamp }
    }
    
    fn from_str(s: &str, initial_timestamp: u128) -> Result<Self, &'static str> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.len() != 2 {
            return Err("Invalid data format");
        }
        
        let temperature = parts[0].parse().map_err(|_| "Invalid temperature")?;
        let humidity = parts[1].parse().map_err(|_| "Invalid humidity")?;
        Ok(Self::new(temperature, humidity, initial_timestamp))
    }
    
    fn to_csv_line(&self) -> String {
        format!("{},{},{}", self.temperature, self.humidity, self.incremental_timestamp)
    }
}


fn main() {
    let initial_timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
    let mut port = serialport::new("/dev/tty.usbserial-1410", 115_200)
        .timeout(Duration::from_millis(10))
        .open().expect("Failed to open port");

    let mut serial_buf: Vec<u8> = Vec::new();
    let mut temp_buf = [0; 32]; // Temporary buffer for each read
    let path = create_csv(initial_timestamp).expect("Failed to create CSV file");


    loop {
        match port.read(&mut temp_buf) {
            Ok(bytes_read) if bytes_read > 0 => {
                serial_buf.extend_from_slice(&temp_buf[..bytes_read]);
                if let Some(measurement) = process_buffer(&mut serial_buf, initial_timestamp) {
                    if let Err(e) = append_to_csv(&path, &measurement.to_csv_line()) {
                        eprintln!("Error writing to CSV: {:?}", e);
                    }
                }
            },
            Ok(_) => (),
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => eprintln!("Serial read error: {:?}", e),
        }
        thread::sleep(Duration::from_millis(100));
    }
}

fn create_csv(initial_timestamp: u128) -> std::io::Result<String> {
    let filename = format!("{}.csv", initial_timestamp);
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(filename.clone())?;

    if let Err(e) = writeln!(file, "temperature,humidity,incremental_timestamp") {
        eprintln!("Couldn't write to file: {}", e);
        return Err(e)
    }
    Ok(filename)
}

fn append_to_csv(path: &str, line: &str) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(path)?;

    if let Err(e) = writeln!(file, "{}", line) {
        eprintln!("Couldn't write to file: {}", e);
        return Err(e)
    }
    Ok(())
}   

fn process_buffer(buf: &mut Vec<u8>, initial_timestamp: u128) -> Option<Measurement> {
    if let Some(pos) = buf.iter().position(|&x| x == b'\n') {
        let line = buf.drain(..=pos).collect::<Vec<u8>>();
        if let Ok(str_line) = String::from_utf8(line) {
            return Measurement::from_str(str_line.trim(), initial_timestamp).ok();
        }
    }
    None
}
