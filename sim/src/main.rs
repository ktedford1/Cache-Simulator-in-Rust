use std::env;
use getopt::Opt;
use std::error::Error;
//use std::ops::Div;		// necessary for the right shift operation??
//use std::ops::Shr;		// necessary for right shift or for modulo??

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
 

// am I supposed to cite the class lab for this code?

 if let Ok(file) = File::open(filename) {
        let reader = io::BufReader::new(file);
				let mut hits, misses, evictions == 0;
        for line in reader.lines() {
            if let Ok(line_content) = line {
                split the line between the operation letter and the hex address
								trim white space
								let mut operation: String = ..
								let mut hex_address: String = .. {
                if operation == "I"
											break}
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

	Ok(())
}


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

fn process_address(&hex_address, &block_bits, &set_bits) -> Result<(u32, u32), Box<dyn Error>> {
		let tag_and_set = hex_address >> &block_bits;
		let tag = tag_and_set / &set_bits;														// store the quotient as tag
		let set_index = tag_and_set % &set_bits;											// store the remainder as set_index
		Ok((tag, set_index))
}

/*
fn update_cache(&cache, &tag, &set, &lines) -> String 

	look in the correct set_index
		look through all the lines in the set:
			if validity bits == 1 and already_there_tag == self.tag:
				update recency
				return HIT (quit the function)

		loop through all the lines in the set:
			if validity bits == 0		// cache still has a space
					add this tag and set_index
					update the recency thing
					change the validity bit to 1
					return MISS (quit the function)
		
		call eviction function and get the tag of the oldest entry
		change the oldest_tag to this self.tag
		update the recency, validity, set_index
		return MISS and EVICTIONS		// can I return 1 or 2 strings from this function? or do I have to specify at the top and stick to it?


fn evict_tag(&cache, &tag, &set, &lines)  -> returns tag of oldest entry

	let current_time = (current_time stamp);
	let mut age = 0;
	let oldest_tag = 0;

	look in the correct set_index
		look through all the lines in the set:
			block_age = current_time - recency info
			if block_age > age:
				age = block_age
				oldest_tag = tag
	
	return oldest_tag

*/

