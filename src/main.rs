use lineio::LineIO;

fn main() {
    let mut lineio = match LineIO::new(&"test.txt".to_string()) {
        Ok(line_reader) => line_reader,
        Err(error) => {panic!("File opening error: {error:?}");}
    };
    loop {
        let s = match lineio.getline() {
            Ok(str) => str,
            Err(_error) => break, // End of input
        };
        println!("Read {}", s);
    }
}
