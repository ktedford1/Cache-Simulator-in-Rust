use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};

pub struct Line {
    tag: u32,
		set_index: u32,
		valid: bool,
		recency: u32,
}

pub struct Set {
	lines: Vec<Line>,
	access_counter: u32,
}

pub struct Cache {
	sets: Vec<Set>,
}

/*
impl Cache {
    // Create a new empty Cache
    pub fn new() -> Self {
        Cache { sets: Vec<Set>::new() }
    }

		required methods for cache:
		calculate 

    // Insert or update a word and its definition in the dictionary
    pub fn insert_word(&mut self, word: &str, definition: &str) {
        self.data.insert(word.to_string(), definition.to_string());
    }

    // Look up the definition of a word in the dictionary
    pub fn lookup_word(&self, word: &str) -> &str {
        match self.data.get(word) {
            Some(definition) => definition,
            None => "Word doesn't exist",
        }
    }

    // Delete a word from the dictionary
    pub fn delete_word(&mut self, word: &str) {
        self.data.remove(word);
    }
}
	let sets_sum = 2_u32.pow(set_bits);							// sets_sum == S == total sets
  let block_size = 2_u32.pow(block_bits);      		// block_size == B == total bytes per block
	let cache_size: u32 = sets_sum * lines * block_size;

