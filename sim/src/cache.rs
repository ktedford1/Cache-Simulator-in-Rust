// use std::...??



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

impl Cache {
    // Create a new empty Cache
    pub fn new() -> Self {
        Cache { sets: Vec<Set>::new() }
    }

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