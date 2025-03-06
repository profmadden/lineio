use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::io::ErrorKind;

pub struct LineIO {
    reader: BufReader<File>,
}

impl LineIO {
    /// Opens the indicated file, and creates a reader to read
    /// in data.
    pub fn new(filename: &String) -> LineIO {
        let f = File::open(filename).unwrap();
        let reader = BufReader::with_capacity(32000, f);
        LineIO {
            reader,
        }
    }

    /// Parses input, skipping lines that begin with a hash
    /// mark (#), and also skipping blank lines.
    /// Returns a single line as a String
    pub fn getline(&mut self) -> std::io::Result<String> {
        loop {
            let mut line = String::new();
            let _len = self.reader.read_line(&mut line).unwrap();
            // println!("Read in {} bytes, line {}", _len, line);
    
            if _len == 0 {
                return std::result::Result::Err(Error::new(ErrorKind::Other, "end of file"));
            }
    
            if line.starts_with("#") {
                // println!("Skip comment.");
                continue;
            }
    
            if _len == 1 {
                continue;
            }
    
            return Ok(line.trim().to_string());
        }
        // Error::new(ErrorKind::Other, "Not reachable FILE IO error");
    }
    /// Convenience function to convert a string into a vector of usize.
    pub fn to_usize(s: &String) -> Vec<usize> {
        s.split_whitespace().map(|v| v.parse().unwrap()).collect() 
    }
    /// Convenience function to convert a string into a vector of f32.
    pub fn to_f32(s: &String) -> Vec<f32> {
        s.split_whitespace().map(|v| v.parse().unwrap()).collect() 
    }
    /// Convenience function to convert a string into a vector of u32.
    pub fn to_u32(s: &String) -> Vec<u32> {
        s.split_whitespace().map(|v| v.parse().unwrap()).collect() 
    }        
    /// Convenience function to convert a string into a vector of i32.
    pub fn to_i32(s: &String) -> Vec<i32> {
        s.split_whitespace().map(|v| v.parse().unwrap()).collect() 
    }    
}

