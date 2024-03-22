use std::env;

pub struct Line {
    tag: u64,
		set_index: u64,
		valid: bool,
		recency: u64,
}

pub struct Set {
	lines: Vec<Line>,
	access_counter: u32,
}

pub struct Cache {
	sets: Vec<Set>,
	total_sets: u64,
	total_lines: u64,
}

impl Cache {

	// Create an empty Cache with the specified dimensions
	pub fn new_cache(sets_sum: u64, line_sum: u64) -> Cache {

		let mut one_line = Line {
			tag: 0,
			set_index: 0,
			valid: false,
			recency: 0,
		}

		let mut index = 0;
		let mut all_lines = Vec::new();
		while index < line_sum {
			all_lines.push(one_line.clone());
			index += 1;
			};
		let mut one_set = Set {
			lines: all_lines,
			access_counter: 0,
		};

		let mut index = 0;
		let mut all_sets = Vec::new();
		while index < sets_sum {
			all_sets.push(one_set.clone());
			index += 1;
		};
		let mut new_cache = Cache {
			sets: all_sets,
			total_sets: sets_sum,
			total_lines: line_sum,
		};
		new_cache
	}

	pub fn update_cache(&mut self, tag_query: &tag, set_query: &set_index) -> String {

		let mut current_set: &Set = &self.sets[set_query];
		let mut current_set.access_counter += 1;
	
		// check if the set_index/tag are already in the cache by looking in the specified set
		// for a line with the same tag and the valid set to true (meaning an entry is stored there)
		for i in 0..self.total_lines {
			if current_set.lines.valid == true && current_set.lines.tag == tag_query {	
				current_set.lines.recency = access_counter;
				return HIT;
			}
		}

		// if there is no hit on the address, check if there is space in the specified set by 
		// checking if any valid bits are still set to false and then entering the tag and changing the valid to true
		for i in 0..self.total_lines {
			if current_set.lines.valid == false {	
				current_set.lines.valid = true;
				current_set.lines.tag = tag_query;					// do I have to clone this? is copying automatic??
				current_set.lines.recency = access_counter;
				return MISS;
			}
		}

		// if there is no hit on the address and there is no space in the specified set
		// then call the eviction function and get the tag of the LRU entry
		// change the lru_tag to the current tag_query - leave valid set to true,
		// update the recency to the current access_counter

		lru_tag = evict_tag(Self, tag_query, set_query);		// do I really need to enter the set_query again??
		for i in 0..self.total_lines {
			if current_set.lines.tag == lru_tag {	
				current_set.lines.tag = tag_query;					// do I have to clone this? is copying automatic??
				current_set.lines.recency = access_counter;
				return MISS EVICTION;				// HOW to return a variable number of results?? with an enum???
			}
		}

	pub fn evict_tag(&mut self, tag_query: &tag, set_query: &set_index) -> String {
		let mut current_set: &Set = &self.sets[set_query];
		let mut lru_tag = current_set.lines[0].recency;
		for i in 0..self.total_lines {
			if current_set.lines[i].recency < lru_tag {
				lru_tag = current_set.lines[i].recency;
			}
		}
		return lru_tag
	}
}
}
