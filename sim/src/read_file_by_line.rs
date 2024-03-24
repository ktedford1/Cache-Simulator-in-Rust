pub mod read_file_by_line {

	use std::fs::File;
	use std::io::{BufReader, BufRead};
	use sim::Cache;
	use std::error::Error;

	pub fn read_file_by_line(new_cache: &mut Cache, filepath: &str, block_bits: &u64, set_bits: &u64) -> Result<(), Box<dyn Error>> {

		let file = File::open(filepath)?;					// use code from Coursework Lab 2, part 20 "Input/Output - Buffers: read a text file line by line"
		let reader = BufReader::new(file);

		for line in reader.lines() {							// go through the trace file one line at a time
			let unwrapped_line = line?;
			let info:Vec<&str> = unwrapped_line.trim().split_whitespace().collect();				// remove any leading or trailing whitespace and split the string in 2 at the space after the operation
			let operation = info[0];								// assign the first item in the vector to a variable called 'operation'
				if operation == "I" {									// ignore any lines with the 'I' for 'instruction' operation, as per the assignment specs
					continue
				}
			let hex_address = info[1].split(',').next().unwrap();					// split the second item in the vector at the comma and keep the first part,the hex_address, and use .next() and .unwrap() because iterator
			let binary_value = u64::from_str_radix(&hex_address, 16)?;		// convert the hex address from the trace file to a binary value so that the tag and set_index can be extracted
			let (tag, set_index) = process_address(&binary_value, block_bits, set_bits);		// see 'fn process_address' below

			// Note: for verbose mode, decomment lines 25 and 34
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


	pub fn process_address(binary_value: &u64, block_bits: &u64, set_bits:&u64) -> (u64, u64) {

		let tag_and_set = binary_value >> block_bits;			// right-shift the binary form of the address by 'b' or 'block_bits' number of bits to get rid of them
		let sets = 2_u32.pow(*set_bits as u32);						// get the total number of sets (from 's', the set_bits) by calculating 2^s
		let sets = sets as u64;

		// split the binary number 'tag_and_set' in 2 parts
		let tag = tag_and_set / sets;											// divide the 'tag_and_set' part of the binary address by the number of sets and store the quotient as 'tag'
		let set_index = tag_and_set % sets;								// divide the 'tag_and_set' part of the binary address by the number of sets and store the remainder as 'set_index'
		(tag, set_index)
	}
}
