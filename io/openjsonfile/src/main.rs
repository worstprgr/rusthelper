use std::fs::File as File;
use std::io::{ BufReader, Read };
use serde_json::from_str;


fn main() {
    const BASE_PATH: &str = "src/";
    let file_name: &str = "test.json";
    let fp: String = format!("{}{}", BASE_PATH, file_name);

    let mut data = String::new();
    let f = File::open(fp).expect("Unable to open file");
    let mut br = BufReader::new(f);
    br.read_to_string(&mut data).expect("Unable to read string");
    println!("{}", data);

    let test: serde_json::Value = from_str(&*data)
        .expect("JSON was not well-formatted");

    println!("{}", test);
}
