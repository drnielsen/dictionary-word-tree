use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    println!("{}", "Opening file");
    let f = File::open("/Users/drnielsen/Development/misc/english-words/words.txt").unwrap();
    let buffered_reader = BufReader::new(f);

    println!("{}", "Finding all 5 word files");
    let mut count = 0;
    for line in buffered_reader.lines() {
        let s = line.unwrap();
        if s.len() == 5 {
            count += 1;
            println!("{}", s);
        }
    }
    println!("Found {} word with 5 letters.", count);
}
