// use std::collections::HashMap;
// note: determine how big the block, tag, and age fields have to be
use std::time::SystemTime;

let sys_time = SystemTime::now();

pub struct Line {
	tag: [u32],
	set_index:[u32],
	valid: [bool],
	recency: [what datatype is sys_time ??? ],
}

pub struct Set {
	line_list: Vec<Line>,
}

pub struct Cache {
		set_list: Vec<Set>, 
}

impl Cache {
    // Create a new empty cache --- how to initialize everything???
    pub fn new() -> Self {
        Cache { set_list: ??? }
    }

    // Insert or update a line
    pub fn insert_line(&mut self, tag: u32, set_index: u32, valid: bool, recency: datatype???) {
        self.line_list.insert(tag, set_index, valid, recency -- how to format this???);
    }

		// modify this dictionary stuff ----->
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

/*
what cache functions do we need?

insert set, lines, a block
lookup tag, set_index, lines
insert_tag, insert_set_index, insert_validity_flag, insert_recency
delete_tag

*/