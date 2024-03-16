use std::env;
use getopt::Opt;
use std::error::Error;
//use std::ops::Div;
//use std::ops::Shr;

fn main() -> Result<(), Box<dyn Error>> {
	
  let args: Vec<String> = env::args().collect();
	let (set_bits, lines, block_bits, trace_file) = parse_args(&args)?;

	let sets_sum = 2_u32.pow(set_bits);							// sets_sum == S == total sets
  let block_size = 2_u32.pow(block_bits);      		// block_size == B == total bytes per block
	let cache_size: u32 = sets_sum * lines * block_size;

  println!("number of set_bits is: {}   number of sets is: {}", set_bits, sets_sum);  
	println!("number of block_bits is: {}   block size is: {}", block_bits, block_size );
  println!("there are {} sets, {} lines per set, and {} bytes per block;", sets_sum, lines, block_size);
	println!("therefore, the cache size in bytes is: {}", cache_size);
	println!("text file is: {}", trace_file);

/*  
	let mut hits, misses, evictions == 0

	open text file
	while text file lines not empty:
		get a line
		split the line between the operation letter and the hex address, and store them
			let mut operation: String = ..
			let mut hex_address: String = ..
		does 'operation' == 'I' ? if so, break
		convert the hex_address to a binary_address
		tag, set_index = process_hex_address(&binary_address, &set_bits, &block_bits)
		let mut attempt: String = update_cache(&cache, &tag, &set_index, &lines)

		if operation == L or S and attempt == HIT: hits += 1
		if operation == L or S and attempt == MISS: misses += 1
		if operation == M and attempt == HIT: hits += 2
		if operation == M and attempt == MISS: misses += 1 and hits += 1
		if eviction: evictions += 1

	end of loop

	println!(“Hits: {0} Misses: {1} Evictions: {2}, hits, misses, evictions);
*/

	Ok(())
}

// Note: why do they mention the E=1 direct-mapped cache? am I supposed to do something with this
// or is it just to help us understand things??

#[allow(unused)]
fn parse_args(args: &Vec<String>) -> Result<(u32, u32, u32, String), Box<dyn Error>> {
	let mut opts = getopt::Parser::new(&args, "hvs:E:b:t:");
  let mut h = false;						// help flag
  let mut v = false;						// verbose flag
  let mut s = String::new();		// set bits
  let mut e = String::new();		// total lines per set
  let mut b = String::new();		// block bits
  let mut t = String::new();		// filepath to text file of Valgrind memory trace; add valid. to make sure it's a real file??

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
        _ => return Err("whatever".into()),
      },
    }
  }

	let s: u32 = s.parse()?;			// convert the arg 's' from a string to an int
	let e: u32 = e.parse()?;			// convert the arg 'e' from a string to an int
	let b: u32 = b.parse()?;			// convert the arg 'b' from a string to an int

  if h || s < 1 || e < 1 || t.is_empty() {
    print_usage_msg();
		return Err("other problem".into());
  }
	Ok((s, e, b, t))
}

fn print_usage_msg() {
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

/*
fn process_binary_address(&binary_address, &tag, &set, &lines) -> Result<(u32, u32), Box<dyn Error>> {

		tag_plus_set = right shift binary_addr >> &block_bits;		// can I right shift binary or does it have to be an integer?
		tag = tag_plus_set / &set_bits														// store the quotient as tag
		set_index = tag_plus_set % &set_bits											// store the remainder as set_index
		Ok((tag, set_index))
}

fn update_cache(&cache, &tag, &set, &lines)
   
	 		look in the cache at that set and check if it has space:
				loop through all the validity bits: 
					is any validity bit == 0 ?
						y -- cache_has_space
						n -- chache_is_full
					
					if cache_has_space:
						loop through all the validity_bits == 1:
							is this any tag that matches this one? --> 			// check the logic here - something is not quite right
								update the recency tag												// if the addresses have no tag - then what?
								return "HIT"
							is this tag not there? -->
								add this tag to the set with the set_index
								update the validity tag from 0 to 1
								update the recency tag
							return "MISS"

						if cache_is_full:											// note: duplication of code here - fix the flow!!
							call the eviction function					// it should kick one out according to LRU
							add this tag to the set 						// look for the validity tag == 0
							update the validity tag from 0 to 1
							update the recency tag
							return "MISS"
					

// note: figure out what to do if there is no tag, as in the coursework sample
// there were 8 bits, 4 for the block, 4 for the set, so the tag had no bits


fn evict(cache, address, policy) -> returns nothing (but modifies the cache)

  if policy == FIFO:
    find the address with the oldest age tag
    change it to the new address and update it’s age tag
  if policy == LRU:
    find the address with the oldest recency tag
    change it to the new address and update it’s recency tag

NOTE: if the policy will be “FIFO”, then the age tag is used
NOTE: if the policy will be “LRU”, then the recency tag is used
NOTE: tutor says that we only need to do 'LRU' ????

*/

