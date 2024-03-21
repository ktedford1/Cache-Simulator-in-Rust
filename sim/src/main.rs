use std::env;
use std::fs::File;
use std::error::Error;
use std::io::{BufReader, BufRead};
use crate::parse_args::parse_args::*;

mod parse_args;

fn main() -> Result<(), Box<dyn Error>> {
	
  let args: Vec<String> = env::args().collect();
	let (set_bits, lines, block_bits, trace_file) = parse_them(&args)?;
	println!("trace file is: {}", trace_file);
	println!("set_bits and block_bits are: {}, {}", set_bits, block_bits);

	let block_bits = block_bits as u64;
	let set_bits = set_bits as u64;
	let _d = read_file_by_line(&trace_file, &block_bits, &set_bits);

	Ok(())
}

fn read_file_by_line(filepath: &str, block_bits: &u64, set_bits: &u64) -> Result<(), Box<dyn Error>> {

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
	}

	Ok(())
}

/*
				let mut hits, misses, evictions == 0;
  
								tag, set_index = process_address(&hex_address, &set_bits, &block_bits)
								let mut attempt: String = update_cache(&cache, &tag, &set_index, &lines)
								if operation == L or S and attempt == HIT: hits += 1
								if operation == L or S and attempt == MISS: misses += 1
								if operation == M and attempt == HIT: hits += 2
								if operation == M and attempt == MISS: misses += 1 and hits += 1
								if EVICTION: evictions += 1

                    if verbose {
                        println!("{}:{}: {}", operation, hex_address, attempt (has to also include eviction!!));
                    }
                }
            }
        }

        println!(“Hits: {} Misses: {} Evictions: {}, hits, misses, evictions);
    } else {
        eprintln!("Error opening the file '{}'", filename);
    
note: for stats, count the operations:L and S each count for 1, M counts for 2

*/


/*
fn update_cache(cache: &cache, new_tag: &tag, new_set: &set, total_lines: &lines) -> String 

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


fn evict_tag(&cache, &set, &lines)  -> returns smallest_recency_tag

	smallest_recency_tag = set.line[0].recency
	loop through all the lines in the set:
			if line.recency < smallest_recency_tag:
				smallest_recency_tag = line.recency
	
	return smallest_recency_tag

*/

