use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::io::ErrorKind;

pub struct LineIO {
    reader: BufReader<File>,
}

impl LineIO {
    pub fn new(filename: &String) -> LineIO {
        let f = File::open(filename).unwrap();
        let reader = BufReader::with_capacity(32000, f);
        LineIO {
            reader,
        }
    }

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
}

