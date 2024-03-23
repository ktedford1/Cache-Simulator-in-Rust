use std::env;
use std::fs::File;
use std::error::Error;
use std::io::{BufReader, BufRead};
use crate::parse_args::parse_args::*;

mod parse_args;


fn main() -> Result<(), Box<dyn Error>> {
	
  let args: Vec<String> = env::args().collect();
	let (set_bits, line_sum, block_bits, trace_file) = parse_them(&args)?;
	let sets_sum = 2_u32.pow(set_bits);							// sets_sum == S == total sets
  let block_size = 2_u32.pow(block_bits);      		// block_size == B == total bytes per block
	let cache_size: u32 = sets_sum * line_sum * block_size;
	println!("sets_sum, block_size, and cache_size are {}, {}, and {}", sets_sum, block_size, cache_size);

	let set_bits = set_bits as u64;
	let block_bits = block_bits as u64;
	let mut new_cache = sim::Cache::new_cache(sets_sum, line_sum);
	
	let mut hits = 0;
	let mut misses = 0;
	let mut evictions = 0;
	
	let _d = read_file_by_line(&new_cache, &trace_file, &block_bits, &set_bits);



	println!("hits: {} misses: {} evictions: {}", hits, misses, evictions);

	// verbose? stats?

	Ok(())
}

// change the return type to a string!!!
fn read_file_by_line(new_cache: &Cache, filepath: &str, block_bits: &u64, set_bits: &u64) -> Result<String, Box<dyn Error>> {

	let file = File::open(filepath)?;					// use code from Coursework Lab 2, part 20 "Input/Output - Buffers: read a text file line by line"
	let reader = BufReader::new(file);

	for line in reader.lines() {							// go through the trace file one line at a time
		let unwrapped_line = line?;
		let info:Vec<&str> = unwrapped_line.trim().split_whitespace().collect();				// remove any leading or trailing whitespace and split the string in 2 at the space after the operation
		let operation = info[0];								// assign the first item in the vector to a variable called operation
			if operation == "I" {									// ignore any lines with the 'instruction' operation, as per the assignment specs
				continue
			}
		let hex_address = info[1].split(',').next().unwrap();					// split the second item in the vector at the comma and keep the first part,the hex_address, and use .next() and .unwrap() because iterator
		let binary_value = u64::from_str_radix(&hex_address, 16)?;
		
		let (tag, set_index) = process_address(&binary_value, block_bits, set_bits);

		let new_entry: String = new_cache.update_cache(tag, set_index);



	if new_entry == "Hit" && (operation == "L" || operation == "S") {
		hits += 1;
	}
		else if new_entry == "Miss" && (operation == "L" || operation == "S") {
			misses += 1;
		}
		else if new_entry == "MissEviction" && (operation == "L" || operation == "S") {
			misses += 1;
			evictions += 1;
		}
		else if new_entry == "Hit" && (operation == "M") {
			hits += 2;
		}
		else if new_entry == "Miss" && (operation == "M") {
			misses += 1;
			hits += 1;	
		}
		else if new_entry == "MissEviction" && (operation == "M") {
			misses += 1;
			hits += 1;
			evictions += 1;
		}


		}
	
	Ok((new_entry))
}




