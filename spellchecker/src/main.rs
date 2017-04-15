extern crate regex;

use regex::Regex;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::io::{Read,BufReader,BufRead,stdout,Write,stdin};

fn main() {
    let args = env::args().collect::<String>();
    let mut hmap = read_corpus(&args).unwrap();
    write_output(stdout(), &hmap);

}

fn read_corpus(filename: &str) -> std::io::Result<HashMap<String, usize>>{
	let file = File::open("corpus.txt")?;
	let mut hmap = HashMap::<String, usize>::new();
	let mut buf_reader = BufReader::new(file);
	let mut contents = String::new();
	buf_reader.read_to_string(&mut contents)?;
	for c in contents.split(" ") {
  		let val = hmap.entry(c.to_string()).or_insert(0);

        // add 1 to value
        *val += 1;
	}
	Ok(hmap)
}


fn write_output<W: Write>(mut writer: W, hm: &HashMap<String, usize> ) {
	for (k, v) in hm {
		writeln!(writer, "{}: {}", k, v).unwrap();
	}
	
}