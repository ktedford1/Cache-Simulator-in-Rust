use std::env;
use std::error::Error;
use crate::parse_args::parse_args::*;
use crate::read_file_by_line::read_file_by_line::*;

mod parse_args;
mod read_file_by_line;

fn main() -> Result<(), Box<dyn Error>> {
	
	// collect the command line arguments (CLI) into a vector
  let args: Vec<String> = env::args().collect();

	// parse the CLI with the module 'parse_args' in the file called: sim/src/parse_args.rs
	let (set_bits, line_sum, block_bits, trace_file) = parse_them(&args)?;

	// create a new instance of an empty cache with the specified number of sets and lines using the 'Cache' struct in the file: sim/src/lib.rs
	let sets_sum = 2_u32.pow(set_bits as u32);
	let mut new_cache = sim::Cache::new_cache(sets_sum, line_sum);

	// read the trace file line by line; parse the operation, parse the set_index and tag from the address; update the cache
	let _d = read_file_by_line(&mut new_cache, &trace_file, &block_bits, &set_bits);
	println!("hits:{} misses:{} evictions:{}", new_cache.hits, new_cache.misses, new_cache.evictions);

	Ok(())
}
