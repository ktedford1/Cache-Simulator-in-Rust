use std::env;
use std::error::Error;
use crate::parse_args::parse_args::*;
use crate::read_file_by_line::read_file_by_line::*;

mod parse_args;
mod read_file_by_line;

fn main() -> Result<(), Box<dyn Error>> {
	
  let args: Vec<String> = env::args().collect();
	let (set_bits, line_sum, block_bits, trace_file) = parse_them(&args)?;
	let sets_sum = 2_u32.pow(set_bits as u32);

	let mut new_cache = sim::Cache::new_cache(sets_sum, line_sum);
	let _d = read_file_by_line(&mut new_cache, &trace_file, &block_bits, &set_bits);
	println!("hits:{} misses:{} evictions:{}", new_cache.hits, new_cache.misses, new_cache.evictions);
	Ok(())
}
