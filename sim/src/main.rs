use std::env;
use std::fs::File;
use std::error::Error;
use std::io::{BufReader, BufRead};
use crate::parse_args::parse_args::*;

mod parse_args;


fn main() -> Result<(), Box<dyn Error>> {
	
  let args: Vec<String> = env::args().collect();
	let (set_bits, line_sum, block_bits, trace_file) = parse_them(&args)?;
	println!("trace file is: {}", trace_file);
	println!("set_bits and block_bits are: {}, {}", set_bits, block_bits);

	let block_bits = block_bits as u64;
	let set_bits = set_bits as u64;
	let sets_sum = 2_u32.pow(set_bits);							// sets_sum == S == total sets
  let block_size = 2_u32.pow(block_bits);      		// block_size == B == total bytes per block
	let cache_size: u32 = sets_sum * line_sum * block_size;

	let mut new_cache = Cache::new(sets_sum, line_sum);

	let mut hits, misses, evictions == 0;

	let new_entry: String = read_file_by_line(&new_cache, &trace_file, &block_bits, &set_bits);

	if new_entry == "HIT" && (operation == "L" || operation == "S") {
		hits += 1;
	}
		else if new_entry == "MISS" && (operation == "L" || operation == "S") {
			misses += 1
		}
		else if new_entry == "HIT" && (operation == "M") {
			hits += 2
		}
		else if new_entry == "MISS" && (operation == "M") {
			misses += 1;
			hits += 1;
		}
		// if evictions then evictions += 1
		// add verbose mode for checking?
		// add final print statement
		// count up memory accesses for stats - was there a method??

	Ok(())
}

// change the return type to a string!!!
fn read_file_by_line(new_cache: &Cache, filepath: &str, block_bits: &u64, set_bits: &u64) -> Result<(), Box<dyn Error>> {

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
		print!("Hex_address: {}  ", hex_address);
		let binary_value = u64::from_str_radix(&hex_address, 16).expect("trouble with converting hex_address");
		let (tag, set_index) = process_address(&binary_value, &block_bits, &set_bits);

		let new_entry: String = new_cache.update_cache(tag, set_index);

		}

	// return new_entry !!!
	
	Ok(())
}




