use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::io::{Read,BufReader,BufRead,stdout,Write,stdin};
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
    let output = spell_check(&input, &corpus);
    for w in output {
    	println!("{}", w);
    }
}

fn read_corpus(filename: &str) -> std::io::Result<HashMap<String, usize>>{
	let file = File::open(filename)?;
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

fn spell_check(vec_str: &Vec<String>, corpus: &HashMap<String, usize>) -> Vec<String> {
	let mut output_vec = vec![];
	for check_word in vec_str {
		if corpus.contains_key(check_word) {
			let mut print_word = check_word.to_string();
			print_word.push_str("\n");
			output_vec.push(print_word);
		} else {
			let mut one_edit_vec = create_variations(&check_word);
			let mut wordfreq_vec = Vec::<WordFreq>::new();
			for edited_word in one_edit_vec {
				if corpus.contains_key(&edited_word) {
					wordfreq_vec.push( WordFreq { freq: corpus[&edited_word], word: edited_word.clone()});
				}
				let mut two_edit_vec = create_variations(&edited_word);
				for edited_word2 in two_edit_vec {
					if corpus.contains_key(&edited_word2) {
						wordfreq_vec.push( WordFreq { freq: corpus[&edited_word2], word: edited_word2});
					}
				}
			}
			let mut output_str = check_word.clone();
			output_str.push_str(", ");
			if wordfreq_vec.is_empty() {
				output_str.push_str("-");
			} else {
				wordfreq_vec.sort();
				output_str.push_str(&wordfreq_vec[0].word);
			}
			output_str.push_str("\n");
			output_vec.push(output_str.to_string());
		}
	}
	output_vec
}


fn create_variations(word: &String) -> Vec<String> {
	let mut vec_variations = vec![];
	let mut temp_vec = delete_edit(&word);
	vec_variations.append(&mut temp_vec);
	temp_vec = transpose_edit(&word);
	vec_variations.append(&mut temp_vec);
	temp_vec = replace_edit(&word);
	vec_variations.append(&mut temp_vec);
	temp_vec = insert_edit(&word);
	vec_variations.append(&mut temp_vec);

	// vec_variations.append(delete_edit(&word));
	// vec_variations.append(transpose_edit(&word));
	// vec_variations.append(replace_edit(&word));
	// vec_variations.append(insert_edit(&word));

	vec_variations
}

fn delete_edit(word: &str) -> Vec<String> {
	let mut output_vec = vec![];
	for i in 0..word.len() {
		output_vec.push(word[..i].to_string() + &word[i+1..]);
	}
	output_vec
}

fn transpose_edit(word: &str) -> Vec<String> {
	let mut output_vec = vec![];
	let length = word.len();
	// println!("{}", word.to_string());
	if length < 2 {
		return output_vec;
	}
	for i in 0..length-1 {
		output_vec.push(word[..i].to_string() + &word[i+1..i+2] + &word[i..i+1] + &word[i+2..]);
	}
	output_vec
}

fn replace_edit(word: &str) -> Vec<String> {
	let mut output_vec = vec![];
	let alphabet = "abcdefghijklmnopqrstuvwxyz";
	for i in 0..word.len() {
		for letter_i in 0..alphabet.len() {
			output_vec.push(word[..i].to_string() + &alphabet[letter_i..letter_i+1] + &word[i+1..]);
		}
	}
	
	output_vec
}

fn insert_edit(word: &String) -> Vec<String> {
	let mut output_vec = vec![];
	let alpha = "abcdefghijklmnopqrstuvwxyz";
	let alpha_length = alpha.len();
	let length = word.len() + 1;
	
	for i in 0..length {
		let (first, second) = word.split_at(i);
		for j in 0..alpha_length {
			output_vec.push(first.to_string() + &alpha[j..j+1] + &second);
		}
	}

	output_vec
}


fn write_output<W: Write>(mut writer: W, vec: &Vec<String> ) {
	for word in vec {
		writeln!(writer, "{}", word).unwrap();
	}
	
}