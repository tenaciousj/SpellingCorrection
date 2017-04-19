/**
* spellchecker
* a program that finds possible corrections for misspelled words
* Consumes a training file on the command line
* then reads words—one per line—from standard input
*
* Assumptions
* 1) Punctuation is stripped from input
* 2) Corpus has no special formatting, exclusively words separated by spaces
* 2) EOF stops the program (cmd+d on Mac)
* 3) converts all input to lowercase
*/
extern crate regex;

use regex::Regex;
use std::env;
use std::fs::File;
use std::collections::HashMap;
use std::io::{Read,BufReader,BufRead,stdout,Write,stdin,Result};
use std::cmp::Ordering;

#[derive(Clone, Eq, PartialEq, Debug)]
struct WordFreq {
	word: String,
	freq: usize,
}

impl Ord for WordFreq {
	fn cmp(&self, other: &WordFreq) -> Ordering {
		// sort numerically descending
		let eq = other.freq.cmp(&self.freq);
		// sort alphabetically ascending
		if eq == Ordering::Equal {
			return self.word.cmp(&other.word);
		}
		eq
	}
}

impl PartialOrd for WordFreq {
	fn partial_cmp(&self, other: &WordFreq) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
    	println!("usage: spellchecker corpus.txt");
    	return
    }
    let corpus_file = &args[1];
    let corpus = read_corpus(&corpus_file).unwrap();
    let input = read_input(stdin());
    for word in input {
    	write_output(stdout(), &spell_check(&word, &corpus));
    }
}

fn read_corpus(filename: &str) -> Result<HashMap<String, usize>> {
	//read file if it exists
	let file = File::open(filename)?;

	//create hashmap for corpus
	let mut hmap = HashMap::<String, usize>::new();
	let re = Regex::new(r"[^a-z\s]").unwrap();


	//fill corpus hashmap
	let mut buf_reader = BufReader::new(file);
	let mut contents = String::new();
	buf_reader.read_to_string(&mut contents)?;
	let clean_contents = contents.trim().to_lowercase();
	let cleaner_contents = re.replace_all(&clean_contents, "");
	let split_contents = cleaner_contents.split_whitespace();
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
	let re = Regex::new(r"[^a-z]").unwrap();

	while let Some(Ok(line)) = lines.next() {
		let clean_line = line.trim().to_lowercase();
		let word = re.replace_all(clean_line.as_str(), "");
		if word.len() > 0 {
			words.push(word.to_string());
		}
	}
	words
}

fn spell_check(word_to_check: &str, corpus: &HashMap<String, usize>) -> String {
	//if word is spelled correctly
	if corpus.contains_key(word_to_check) {
		word_to_check.to_string()
	}
	//otherwise, try editing once
	else {
		//create all one edit variations
		let one_edit_vec = create_variations(&word_to_check);

		//create word-frequency vector that we will sort later
		let mut wordfreq_vec = Vec::<WordFreq>::new();

		//look thru all one edit variations
		for edited_word in one_edit_vec {
			if corpus.contains_key(&edited_word) {
				wordfreq_vec.push( WordFreq { freq: corpus[&edited_word], word: edited_word.clone()});
			}

			//create all two edit variations
			let two_edit_vec = create_variations(&edited_word);

			//look thru all two edit variations
			for edited_word2 in two_edit_vec {
				if corpus.contains_key(&edited_word2) {
					wordfreq_vec.push( WordFreq { freq: corpus[&edited_word2], word: edited_word2});
				}
			}
		}
		let mut output_str = word_to_check.to_string();
		output_str.push_str(", ");
		//if no matches
		if wordfreq_vec.is_empty() {
			output_str.push_str("-");
		}
		//otherwise, get most probable match
		else {
			wordfreq_vec.sort();
			output_str.push_str(&wordfreq_vec[0].word);
		}
		output_str
	}

}


fn create_variations(word: &str) -> Vec<String> {
	let mut vec_variations = vec![];
	let mut temp_vec = vec![];
	delete_edit(&word, &mut temp_vec);
	vec_variations.append(&mut temp_vec);
	transpose_edit(&word, &mut temp_vec);
	vec_variations.append(&mut temp_vec);
	replace_edit(&word, &mut temp_vec);
	vec_variations.append(&mut temp_vec);
	insert_edit(&word, &mut temp_vec);
	vec_variations.append(&mut temp_vec);


	vec_variations
}

fn delete_edit(word: &str, output_vec: &mut Vec<String>) {
	for i in 0..word.len() {
		output_vec.push(word[..i].to_string() + &word[i+1..]);
	}
}

#[cfg(test)]
mod delete_edit_tests {
	use super::delete_edit;
	// #[test]
	// fn empty_input() {

	// }

	fn assert_delete(expected_output: &[&str], input: &str) {
		
	}
}

fn transpose_edit(word: &str, output_vec: &mut Vec<String>) {
	if word.len() <= 1 {
		return;
	}
	for i in 0..word.len()-1 {
		output_vec.push(word[..i].to_string() + &word[i+1..i+2] + &word[i..i+1] + &word[i+2..]);
	}

}

fn replace_edit(word: &str, output_vec: &mut Vec<String>) {
	let alphabet = "abcdefghijklmnopqrstuvwxyz";
	for i in 0..word.len() {
		for letter_i in 0..alphabet.len() {
			output_vec.push(word[..i].to_string() + &alphabet[letter_i..letter_i+1] + &word[i+1..]);
		}
	}
}

fn insert_edit(word: &str, output_vec: &mut Vec<String>) {
	let alpha = "abcdefghijklmnopqrstuvwxyz";
	let alpha_length = alpha.len();
	let length = word.len() + 1;
	
	for i in 0..length {
		let (first, second) = word.split_at(i);
		for j in 0..alpha_length {
			output_vec.push(first.to_string() + &alpha[j..j+1] + &second);
		}
	}
}


fn write_output<W: Write>(mut writer: W, line: &str ) {
	writeln!(writer, "{}", line).unwrap();
}