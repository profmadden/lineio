//! LineIO is a light-weight line-by-line input library, with
//! the ability to skip over lines that begin with a hash mark,
//! or that are blank.
//! 
//! As part of my research, I deal with a lot of input files that
//! have data organized by line, and with intermixed blank lines
//! and comments.  Rather than rebuilding a parser input each time,
//! the functionality is wrapped up in the crate.
//! 
//! There are also convenience functions to turn a line into either
//! a vector of floating point numbers, or a vector of usize, i32, or u32.
//! 
//! While the library is called LineIO, it currently only handles input.
//! 
//! You can use this as follows:
//! ```
//! use lineio::LineIO;
//! 
//! fn main() {
//!    let mut lineio = LineIO::new(&"test.txt".to_string());
//!    let s = lineio.getline().unwrap();
//!     println!("Read {}", s);
//! }
//! ```

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::io::ErrorKind;

/// The main structure, which includes an internal buffered reader,
/// and the file handle.
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
    /// Returns a single line as a String result, or an
    /// error condition (indicating end of file).
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

