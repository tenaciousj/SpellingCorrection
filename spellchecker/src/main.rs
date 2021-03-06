/**
* spellchecker
* a program that finds possible corrections for misspelled words
* Consumes a training file on the command line
* then reads words—one per line—from standard input
*
* Assumptions
* 1) Punctuation is stripped from input
* 2) Corpus has no special formatting, exclusively words separated by spaces
* 3) EOF stops the program (cmd+d on Mac)
* 4) Converts all input to lowercase
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
    read_spellcheck_input(stdin(), &corpus);
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

fn read_spellcheck_input<R: Read>(reader: R, corpus: &HashMap<String, usize>) {
	let mut lines = BufReader::new(reader).lines();
	let re = Regex::new(r"[^a-z]").unwrap();

	while let Some(Ok(line)) = lines.next() {
		let clean_line = line.trim().to_lowercase();
		let word = re.replace_all(clean_line.as_str(), "");
		if word.len() > 0 {
			//spell check as soon as you read the input
			write_output(stdout(), &spell_check(&word, &corpus));
		}
	}
}

fn spell_check(word_to_check: &str, corpus: &HashMap<String, usize>) -> (String, String) {
	//if word is spelled correctly
	if corpus.contains_key(word_to_check) {
		(word_to_check.to_string(), "".to_string())
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

		//formatting output string
		if wordfreq_vec.is_empty() {
			(word_to_check.to_string(), "-".to_string())
		} else {
			wordfreq_vec.sort();
			let mut output_str = "".to_string();
			output_str.push_str(&wordfreq_vec[0].word);
			(word_to_check.to_string(), output_str)
		}
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
	if word.len() == 1 { return; }
	for i in 0..word.len() {
		output_vec.push(word[..i].to_string() + &word[i+1..]);
	}
}

#[cfg(test)]
mod delete_edit_tests {
	use super::delete_edit;
	#[test]
	fn empty_input() {
		assert_delete(&[], "");
	}

	#[test]
	fn one_input() {
		assert_delete(&[], "x");
	}

	#[test]
	fn two_input() {
		assert_delete(&["y","x"], "xy");
	}

	#[test]
	fn three_input() {
		assert_delete(&["yz","xz","xy"], "xyz");
	}

	fn assert_delete(expected_output: &[&str], input: &str) {
		let mut output = vec![];
		delete_edit(input, &mut output);
		assert!(expected_output.len()==output.len());
		let iter = output.iter().zip(expected_output.iter());
		for (o, eo) in iter {
			assert_eq!(o, eo);
		}

		
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

#[cfg(test)]
mod transpose_edit_tests {
	use super::transpose_edit;
	#[test]
	fn empty_input() {
		assert_transpose(&[], "");
	}

	#[test]
	fn one_input() {
		assert_transpose(&[], "x");
	}

	#[test]
	fn two_input() {
		assert_transpose(&["yx"], "xy");
	}

	#[test]
	fn three_input() {
		assert_transpose(&["yxz","xzy"], "xyz");
	}

	fn assert_transpose(expected_output: &[&str], input: &str) {
		let mut output = vec![];
		transpose_edit(input, &mut output);
		assert!(expected_output.len()==output.len());
		let iter = output.iter().zip(expected_output.iter());
		for (o, eo) in iter {
			assert_eq!(o, eo);
		}

		
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

#[cfg(test)]
mod replace_edit_tests {
	use super::replace_edit;
	#[test]
	fn empty_input() {
		assert_replace("", "");
	}

	#[test]
	fn one_input() {
		let alphabet = "abcdefghijklmnopqrstuvwxyz";
		assert_replace(&alphabet, "x");
	}

	fn assert_replace(expected_output: &str, input: &str) {
		let mut output = vec![];
		replace_edit(input, &mut output);
		assert!(expected_output.len()==output.len());
		let iter = output.iter().zip(expected_output.chars());
		for (o, eo) in iter {
			let ch = o.chars().nth(0).unwrap();
			assert_eq!(ch, eo);
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

#[cfg(test)]
mod insert_edit_tests {
	use super::insert_edit;
	#[test]
	fn empty_input() {
		assert_insert(&["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", 
			"k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", 
			"x", "y", "z"], "");
	}

	#[test]
	fn one_input() {
		assert_insert(&["ax", "bx", "cx", "dx", "ex", "fx", "gx", "hx", 
			"ix", "jx", "kx", "lx", "mx", "nx", "ox", "px", "qx", "rx", "sx", 
			"tx", "ux", "vx", "wx", "xx", "yx", "zx", "xa", "xb", "xc", "xd", 
			"xe", "xf", "xg", "xh", "xi", "xj", "xk", "xl", "xm", "xn", "xo", 
			"xp", "xq", "xr", "xs", "xt", "xu", "xv", "xw", "xx", "xy", "xz"],
			"x");
	}

	fn assert_insert(expected_output: &[&str], input: &str) {
		let mut output = vec![];
		insert_edit(input, &mut output);
		assert!(expected_output.len()==output.len());
		let iter = output.iter().zip(expected_output.iter());
		for (o, eo) in iter {
			assert_eq!(o, eo);
		}

		
	}
}


fn write_output<W: Write>(mut writer: W, line: &(String, String) ) {
	if line.1.len() < 1 {
		writeln!(writer, "{}", line.0).unwrap();
	} else {
		writeln!(writer, "{}, {}", line.0, line.1).unwrap();
	}
}


#[cfg(test)]
mod write_output_tests {
	use super::write_output;
	use std::io::Cursor;

	#[test]
	fn no_correction_needed() {
		assert_write("hello\n", &("hello".to_string(), "".to_string()));
	}

	#[test]
	fn one_correction() {
		assert_write("hell, hello\n", &("hell".to_string(), "hello".to_string()));
	}

	#[test]
	fn no_match() {
		assert_write("w, -\n", &("w".to_string(), "-".to_string()));
	}


	fn assert_write(expected: &str, given: &(String, String)) {
		let mut writer = Cursor::new(vec![]);
		write_output(&mut writer, given);
		assert_eq!(expected.as_bytes(), &*writer.into_inner());
	}

}