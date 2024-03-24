// define a Cache struct for the cache memory simulator
#[derive(Debug, Clone)]
pub struct Line {
    pub tag: usize,
		pub valid: bool,
		pub recency: usize,
}

#[derive(Debug, Clone)]
pub struct Set {
	pub lines: Vec<Line>,
	pub access_counter: usize,
}

#[derive(Debug, Clone)]
pub struct Cache {
	pub sets: Vec<Set>,
	pub hits: usize,
	pub misses: usize,
	pub evictions: usize,
}


impl Cache {

	// Create an empty Cache with the specified dimensions
	pub fn new_cache(sets_sum: usize, line_sum: usize) -> Cache {

		let one_line = Line {
			tag: 0,
			valid: false,
			recency: 0,
		};

		let all_lines = vec![one_line; line_sum];
		let one_set = Set {
			lines: all_lines,
			access_counter: 0,
		};
		
		let all_sets = vec![one_set; sets_sum];
		let new_cache = Cache {
			sets: all_sets,
			hits: 0,
			misses: 0,
			evictions: 0,
		};
		new_cache
	}

	pub fn update_cache(&mut self, tag_query: usize, set_query: usize) {

		// Note: for verbose mode, de-comment the 'print!' lines numbered #66, #78, #101, #109

		// set 'current_set' to the index of the set in the query
		let current_set = &mut self.sets[set_query];

		// to implement the 'LRU' eviction policy: update the counter for the specified set every time there is an access query to the cache
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

	// if the tag_query is not in the set, and there is no line available, then identify the least recently used (LRU) line...
		let mut lru = usize::MAX;
		let mut lru_tag = 0;

		for line in &mut current_set.lines {
			if line.recency < lru {
				lru = line.recency;
				lru_tag = line.tag;
			}
		}

	// ...and	replace it with the current tag_query
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

	// if the operation of the access/query is 'M' for modify, then an extra 'hit' is added to every access
	pub fn extra_hit_for_modify(&mut self) {
		self.hits += 1;
		//print!("hit");
	}
}

pub mod parse_args {

	use getopt::Opt;
	use std::error::Error;

	// code citation: code for 'fn parse_them' was essentially copied from https://docs.rs/getopt/latest/getopt/struct.Parser.html
	// and https://crates.io/crates/getopt and slightly modified to fit the expected arguments for the cache sim

	#[allow(unused)]
	pub fn parse_them(args: &Vec<String>) -> Result<(usize, usize, usize, String), Box<dyn Error>> {
		let mut opts = getopt::Parser::new(&args, "hvs:E:b:t:");
		let mut h = false;						// help flag
		let mut v = false;						// verbose flag	- Note: the verbose function is not set up yet
		let mut s = String::new();		// set bits
		let mut e = String::new();		// total lines per set
		let mut b = String::new();		// block bits
		let mut t = String::new();		// filepath to text file of Valgrind memory trace, e.g. "/home/codio/workspace/traces/yi.trace"

		loop {
			match opts.next().transpose()?{
				None => break,
				Some(opt) => match opt {
					Opt('h', None) => h = true,
					Opt('v', None) => v = true,
					Opt('s', Some(arg)) => s = arg.clone(),
					Opt('E', Some(arg)) => e = arg.clone(),
					Opt('b', Some(arg)) => b = arg.clone(),
					Opt('t', Some(arg)) => t = arg.clone(),
					_ => unreachable!(),
				},
			}
		}

		// to rule out negative numbers as arguments, 's', 'e', and 'b' have to be signed integers, so temporarily convert them to 'i32'
		let s: i32 = s.parse()?;			// convert the arg 's' from a string to an int
		let e: i32 = e.parse()?;			// convert the arg 'e' from a string to an int
		let b: i32 = b.parse()?;			// convert the arg 'b' from a string to an int

		// if help is requested, or if the number of sets or lines is less than 1, or if there is no textfile, print the usage msg
		if h || s < 1 || e < 1 || t.is_empty() {
			print_usage_msg();
		}

		Ok((s as usize, e as usize, b as usize, t))
	}


	pub fn print_usage_msg() {
		println!("Usage:");
		println!("./sim-ref [-hv] -s <s> -E <E> -b <b> -t <tracefile>");
		println!("-h: Optional help flag that prints usage info");
		println!("-v: Optional verbose flag that displays trace info");
		println!("-s <s>: Number of set index bits (S = 2s is the number of sets)");
		println!("-E <E>: Associativity (number of lines per set)");
		println!("-b <b>: Number of block bits (B = 2b is the block size)");
		println!("-t <tracefile>: Name of the trace to replay");
		std::process::exit(0);
	}
}

pub mod read_file_by_line {

	use std::fs::File;
	use std::io::{BufReader, BufRead};
	use crate::Cache;
	use std::error::Error;

	pub fn read_file_by_line(new_cache: &mut Cache, filepath: &str, block_bits: &usize, set_bits: &usize) -> Result<(), Box<dyn Error>> {

		let file = File::open(filepath)?;					// use code from Coursework Lab 2, part 20 "Input/Output - Buffers: read a text file line by line"
		let reader = BufReader::new(file);

		for line in reader.lines() {							// go through the trace file one line at a time
			let unwrapped_line = line?;
			let info:Vec<&str> = unwrapped_line.trim().split_whitespace().collect();				// remove any leading or trailing whitespace and split the string in 2 at the space after the operation
			
			// return an error message if the line does not have 2 parts and exit the program
			match info.len() {
				2 => (),
				_ => {
					println!("'{}' is not a valid trace line. ", unwrapped_line);
					return Err("Trace line format error.".into());
				}
			}
			
			let operation = info[0];								// assign the first item in the vector to a variable called 'operation'
					if operation == "I" {								// ignore any lines with the 'I' for 'instruction' operation, as per the assignment specs
					continue
				}

			// return an error for an invalid operation code and exit the program
			match operation {
				"L" | "S" | "M" => (), 
				_ => {
					println!("Operation code must be 'L', 'S', or 'M'. ");
					return Err("Invalid operation".into());
				}
			}

			// split the second item in the vector at the comma and keep the first part
			let hex_address = info[1].split(',').next().unwrap();
			
			// return an error for an invalid hex address and exit the program
			let binary_value = match u64::from_str_radix(&hex_address, 16) {
				Ok(value) => value,
				Err(_) => {
					println!("'{}' is not a valid hexadecimal address. ", hex_address);
					return Err("Invalid hex address".into());
				}
			};		
			
			// convert the hex address from the trace file to a binary value so that the tag and set_index can be extracted
			let (tag, set_index) = process_address(&binary_value, block_bits, set_bits);		// see 'fn process_address' below

			// Note: for verbose mode, decomment lines 228 and 237
			// print!("{} ", unwrapped_line);
			
			// check the cache: is this tag already present in the specified set (hit)? or not (miss)? if miss, is an eviction required? Update the counts accordingly.
			new_cache.update_cache(tag, set_index);

			// for 'M' operations, an extra 'hit' is added to the counts
			if operation == "M" {
				new_cache.extra_hit_for_modify();
			}
			// println!("");
		}

		Ok(())
	}


	pub fn process_address(binary_value: &u64, block_bits: &usize, set_bits:&usize) -> (usize, usize) {

		let tag_and_set = binary_value >> block_bits;			// right-shift the binary form of the address by 'b' or 'block_bits' number of bits to get rid of them
		let sets = 2_u32.pow(*set_bits as u32);						// get the total number of sets (from 's', the set_bits) by calculating 2^s
		let sets = sets as u64;

		// split the binary number 'tag_and_set' in 2 parts
		let tag = tag_and_set / sets;											// divide the 'tag_and_set' part of the binary address by the number of sets and store the quotient as 'tag'
		let set_index = tag_and_set % sets;								// divide the 'tag_and_set' part of the binary address by the number of sets and store the remainder as 'set_index'
		(tag as usize, set_index as usize)
	}
}
