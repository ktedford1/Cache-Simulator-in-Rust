use std::env;
use std::error::Error;
use sim::parse_args::parse_them;
use sim::read_file_by_line::read_file_by_line;


fn main() -> Result<(), Box<dyn Error>> {
	
	// collect the command line arguments (CLI) into a vector
  let args: Vec<String> = env::args().collect();

	// parse the CLI with the module 'parse_args' in the lib.rs file
	let (set_bits, line_sum, block_bits, trace_file) = parse_them(&args)?;

	// create a new instance of an empty cache with the specified number of sets and lines using the 'Cache' struct in lib.rs
	let sets_sum = (2_u32.pow(set_bits as u32)) as usize;
	let mut new_cache = sim::Cache::new_cache(sets_sum, line_sum);

	// read the trace file line by line; parse the operation, parse the set_index and tag from the address; update the cache
	match read_file_by_line(&mut new_cache, &trace_file, &block_bits, &set_bits) {
		Ok(_) => {},
		Err(_) => {
			return Err(format!("Invalid Valgrind trace file.").into());
		},
	}

	println!("hits:{} misses:{} evictions:{}", new_cache.hits, new_cache.misses, new_cache.evictions);

	Ok(())
}
