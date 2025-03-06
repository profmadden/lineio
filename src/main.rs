use lineio::LineIO;

fn main() {
    let mut lineio = LineIO::new(&"test.txt".to_string());
    let s = lineio.getline().unwrap();
    println!("Read {}", s);
}
