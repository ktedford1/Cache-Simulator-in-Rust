use std::env;

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

	// Create an empty Cache with the specified dimensions
	pub fn new_cache(sets_sum: u64, line_sum: u64, block_size: u64) -> Cache {

		let mut empty_line = Line {
			tag: 0,
			set_index: 0,
			valid: 0,
			recency: 0,
		}

		let mut index = 0;
		let mut empty_lines_in_set = Vec::new();
		while index < line_sum {
			empty_lines_in_set.push(empty_line);
			index += 1;
			}
		let mut empty_set = Set {
			lines: empty_lines_in_set,
			access_counter: 0,
		}

		let mut empty_sets_in_cache = Vec::new();
		while index < sets_sum {
			empty_sets_in_cache.push(empty_set);
			index += 1;
		}
		let mut empty_cache = Cache {
			sets: empty_sets_in_cache,
		}
	}


	pub fn update_cache(&mut self, new_tag: &tag, new_set: &set, line_sum: u64) -> String {

	
	update the set.access_counter += 1;
		loop through all the lines in the set:
			if valid == 1 and set.tag == new_tag:				// when/how do I use self. ?
				recency = access_counter;
				return HIT and exit

		loop through all the lines in the set:
			if valid == 0:													// cache still has a space
					set.tag = new_tag
					recency = access_counter
					valid = 1
					return MISS and exit
		
		lru_tag = evict_tag(&cache, &set, &lines)
		loop through all the lines in the set:
			if line.tag == lru_tag:
				line.tag = lru_tag
				line.recency = 1;
		return MISS and EVICTION		// HOW to return a variable number of results?? with an enum???

}

fn evict_tag(&cache, &set, &lines)  -> returns smallest_recency_tag

	smallest_recency_tag = set.line[0].recency
	loop through all the lines in the set:
			if line.recency < smallest_recency_tag:
				smallest_recency_tag = line.recency
	
	return smallest_recency_tag

*/