extern crate regex;

use regex::Regex;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::io::{Read,BufReader,BufRead,stdout,Write,stdin};

#[derive(Clone, Eq, PartialEq, Debug)]
struct WordFreq {
	freq: usize,
	word: String,
}

fn main() {
    let args = env::args().collect::<String>();
    let corpus = read_corpus(&args).unwrap();
    let input = read_input(stdin());
    write_output(stdout(), &input);


}

fn read_corpus(filename: &str) -> std::io::Result<HashMap<String, usize>>{
	let file = File::open("corpus.txt")?;
	let mut hmap = HashMap::<String, usize>::new();
	let mut buf_reader = BufReader::new(file);
	let mut contents = String::new();
	buf_reader.read_to_string(&mut contents)?;
	let clean_contents = contents.trim().to_lowercase();
	let split_contents = clean_contents.split_whitespace();
	for c in split_contents {
  		let val = hmap.entry(c.to_string()).or_insert(0);
        // add 1 to value
        *val += 1;
	}
	Ok(hmap)
}

fn read_input<R: Read>(reader: R) -> Vec<String> {
	let mut words = vec![];
	let mut lines = BufReader::new(reader).lines();

	while let Some(Ok(line)) = lines.next() {
		let in_words = line.trim().to_lowercase();
		let split_words = in_words.split_whitespace();
		for w in split_words {
			words.push(w.to_string());
		}
	}
	words
}


fn write_output<W: Write>(mut writer: W, vec: &Vec<String> ) {
	for word in vec {
		writeln!(writer, "{}", word).unwrap();
	}
	
}