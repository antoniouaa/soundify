use std::env;
use std::fs;

use soundify;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename);

    let converted = match contents {
        Ok(s) => soundify::convert(&s),
        Err(e) => panic!("ERROR: {}", e),
    };

    soundify::write_to_file(converted).unwrap();
}
