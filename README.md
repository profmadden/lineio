# lineio

Very simple file interface, to parse things a line at a time, skipping blank lines,
and lines with a hash mark in the first column.

Lots of EDA input files are line-by-line formatted, with a fixed structure, and comments
are with the hash mark.  Rather than rebuilding a file reader every time (and having
to include the use descriptions, and whatnot), just a simple wrapper.  Call getline to get
a line, unwrap as needed.  The new function takes a string filename.

## Example

Here's a simple example of reading in a file.

```
use lineio::LineIO;

fn main() {
    let mut lineio = match LineIO::new(&"test.txt".to_string()) {
        Ok(reader) => reader,
        Err(error) => {panic!("File opening error: {error:?}");}
    };
    loop {
        let s = match lineio.getline() {
            Ok(str) => str,
            Err(_error) => break,
        };
        println!("Read {}", s);
    }
}
```
