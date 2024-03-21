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

	let mut new_cache = Cache::new(sets_sum, line_sum, block_size);

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

		let new_entry = new_cache.update_cache(tag, set_index);
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




