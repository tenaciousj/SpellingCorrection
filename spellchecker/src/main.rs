extern crate regex;

use regex::Regex;
use std::env;
use std::io::{Read,BufReader,BufRead,stdout,Write,stdin};

fn main() {
    let args = env::args().collect::<Vec<_>>();
}

fn read_input<R: Read>(reader: R) -> Vec<String> {
	let mut words: Vec<String> = vec![];
	let mut lines = BufReader::new(reader).lines();
	
	let re = Regex::new(r"[^\P{P}-]+").unwrap();

	while let Some(Ok(line)) = lines.next() {
		//strip punctuation
		let sw = re.replace_all(line.as_str(), "").to_string().replace("--", " ");
		//split words between spaces
		let sw_split = sw.split(" ");
		for s in sw_split {
			if !s.trim().is_empty() {
				words.push(s.trim().to_lowercase());
			}
		}
	}

	words
}