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
	total_sets: u32,
	total_lines: u32,
}

pub enum QueryResult {
	Hit,
	Miss,
	MissEviction,
}

impl Cache {

	// Create an empty Cache with the specified dimensions

	pub fn new_cache(sets_sum: u32, line_sum: u32) -> Cache {

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
			total_sets: sets_sum,
			total_lines: line_sum,
		};
		new_cache
	}

	pub fn update_cache(&mut self, tag_query: u64, set_query: u64) -> Option<QueryResult> {

		let current_set = &mut self.sets[set_query as usize];
		current_set.access_counter += 1;
	
		// check if the set_index/tag are already in the cache by looking in the specified set
		// for a line with the same tag and the valid set to true
		for line in &mut current_set.lines {
			if line.valid == true && line.tag == tag_query {	
				line.recency = current_set.access_counter;
				return Some(QueryResult::Hit);
			}
		}

		// if there is no hit on the address, check if there is space in the specified set by 
		// checking if any valid bits are still set to false and then entering the tag and changing the valid to true
		for line in &mut current_set.lines {
			if line.valid == false {	
				line.valid = true;
				line.tag = tag_query;
				line.recency = current_set.access_counter;
				return Some(QueryResult::Miss);
			}
		}

		let mut lru = u32::MAX;
		let mut lru_tag = 0;

		for line in &mut current_set.lines {
			if line.recency < lru {
				lru = line.recency;
				lru_tag = line.tag;
			}
		}
		println!("the lru_tag is: {}", lru_tag);
			
		for line in &mut current_set.lines {
			if line.tag == lru_tag {	
				line.tag = tag_query;
				line.recency = current_set.access_counter;
				return Some(QueryResult::MissEviction);
			}
		}
		None
	} 
}
