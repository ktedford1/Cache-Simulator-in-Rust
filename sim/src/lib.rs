#[derive(Debug, Clone)]
pub struct Line {
    tag: u64,
		valid: bool,
		recency: u32,
}

#[derive(Debug, Clone)]
pub struct Set {
	lines: Vec<Line>,
	access_counter: u32,
}

#[derive(Debug, Clone)]
pub struct Cache {
	sets: Vec<Set>,
	pub hits: u32,
	pub misses: u32,
	pub evictions: u32,
}


impl Cache {

	// Create an empty Cache with the specified dimensions

	pub fn new_cache(sets_sum: u32, line_sum: u64) -> Cache {

		let one_line = Line {
			tag: 0,
			valid: false,
			recency: 0,
		};

		let mut index = 0;
		let mut all_lines = Vec::new();
		while index < line_sum {
			all_lines.push(one_line.clone());
			index += 1;
			};
		let one_set = Set {
			lines: all_lines,
			access_counter: 0,
		};

		let mut index = 0;
		let mut all_sets = Vec::new();
		while index < sets_sum {
			all_sets.push(one_set.clone());
			index += 1;
		};
		let new_cache = Cache {
			sets: all_sets,
			hits: 0,
			misses: 0,
			evictions: 0,
		};
		new_cache
	}

	pub fn update_cache(&mut self, tag_query: u64, set_query: u64) {

		// Note: for verbose mode, decomment lines 72, 84, 106, 113
		let current_set = &mut self.sets[set_query as usize];
		current_set.access_counter += 1;
	
		// is the tag_query in the specified cache set already?
		for line in &mut current_set.lines {
			if line.valid == true && line.tag == tag_query {	
				line.recency = current_set.access_counter;
				self.hits += 1;
				// print!("hit ");
				return
			}
		}

		// if the tag_query is not in the set, is there a line available in the set?
		for line in &mut current_set.lines {
			if line.valid == false {	
				line.valid = true;
				line.tag = tag_query;
				line.recency = current_set.access_counter;
				self.misses += 1;
				// print!("miss ");
				return
			}
		}

	// if the tag_query is not in the set, and there is no line available, then evict the LRU line
		let mut lru = u32::MAX;
		let mut lru_tag = 0;

		for line in &mut current_set.lines {
			if line.recency < lru {
				lru = line.recency;
				lru_tag = line.tag;
			}
		}
				
		for line in &mut current_set.lines {
			if line.tag == lru_tag {	
				line.tag = tag_query;
				line.recency = current_set.access_counter;
				self.misses += 1;
				self.evictions += 1;
				// print!("miss eviction ");
			}
		}
	}

	pub fn extra_hit_for_modify(&mut self) {
		self.hits += 1;
		//print!("hit");
	}
}
