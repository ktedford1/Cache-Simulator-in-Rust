use std::env;
use std::fs::File;
use getopt::Opt;
use std::error::Error;
use std::io::{BufReader, BufRead};
use std::num::ParseIntError;


fn main() -> Result<(), Box<dyn Error>> {
	
  let args: Vec<String> = env::args().collect();
	let (set_bits, lines, block_bits, trace_file) = parse_args(&args)?;

	let sets_sum = 2_u32.pow(set_bits);							// sets_sum == S == total sets
  let block_size = 2_u32.pow(block_bits);      		// block_size == B == total bytes per block
	let cache_size: u32 = sets_sum * lines * block_size;
	println!("trace file is: {}", trace_file);
	println!("sets_sum, block_size, and cache_size are: {}, {}, {}", sets_sum, block_size, cache_size);

	//let (tag, set_index) = process_address(&hex_address, &block_bits, &set_bits);
	let block_bits = block_bits as u64;
	let set_bits = set_bits as u64;
	let _d = read_file_by_line(&trace_file, &block_bits, &set_bits);

	Ok(())
}

fn read_file_by_line(filepath: &str, block_bits: &u64, set_bits: &u64) -> Result<(), Box<dyn Error>> {

	let trace_line = File::open(filepath)?;
	let reader = BufReader::new(trace_line);

	for line in reader.lines() {
		let unwrapped_line = line?;
		let info:Vec<&str> = unwrapped_line.trim().split_whitespace().collect();
		let operation = info[0];
			if operation == "I" {
				continue
			}
		let hex_address = info[1].split(',').next().unwrap();
		print!("Hex_address: {}  ", hex_address);
		let binary_value = hex_to_64_bit_binary(&hex_address).expect("trouble");
		let (tag, set_index) = process_address(&binary_value, &block_bits, &set_bits);
	}

	Ok(())
}

fn hex_to_64_bit_binary(hex_address: &str) -> Result<u64, ParseIntError> {
	u64::from_str_radix(hex_address, 16) 
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

	let s: i32 = s.parse()?;			// convert the arg 's' from a string to an int
	let e: i32 = e.parse()?;			// convert the arg 'e' from a string to an int
	let b: i32 = b.parse()?;			// convert the arg 'b' from a string to an int

// note:it's crashing on "cargo run -- -h" and -v -- check for other potential errors

  if h || s < 1 || e < 1 || t.is_empty() {
    print_usage_msg();
		return Err("other problem".into());
  }
	Ok((s as u32, e as u32, b as u32, t))
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

fn process_address(binary_value: &u64, block_bits: &u64, set_bits:&u64) -> (u64, u64) {

		let tag_and_set = binary_value >> block_bits;
		let sets = 2_u32.pow(*set_bits as u32);	
		let sets = sets as u64;
		let tag = tag_and_set / sets;														// store the quotient as tag
		let set_index = tag_and_set % sets;											// store the remainder as set_index
		println!("tag: {} set: {}", tag, set_index);
		(tag, set_index)
}




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

