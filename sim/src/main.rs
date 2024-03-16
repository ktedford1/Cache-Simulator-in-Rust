use std::env;
use getopt::Opt;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
	
  let args: Vec<String> = env::args().collect();
	let (set_bits, lines, block_bits, trace_file) = parse_args(&args)?;

	let sets_sum = 2_u32.pow(set_bits);      			// S == total sets
  let block_size = 2_u32.pow(block_bits);      		// B == total words per block
	let cache_size: u32 = sets_sum * block_size * lines;

  println!("number of set_bits is: {}   number of sets is: {}", set_bits, sets_sum);  
	println!("number of block_bits is: {}   block size is: {}", block_bits, block_size );
  println!("there are {} sets, {} lines per set, and {} bytes per block;", sets_sum, lines, block_size);
	println!("therefore, the cache size in bytes is: {}", cache_size);
	println!("text file is: {}", trace_file);

  // printsln!(“Hits: {0} Misses: {1} Evictions: {2}, hits, misses, evictions);
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
  let mut t = String::new();		// filepath to text file of Valgrind memory trace

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

	let s: u32 = s.parse()?;								// convert the arg 's' from a string to an int
	let e: u32 = e.parse()?;
	let b: u32 = b.parse()?;							// convert the arg 'b' from a string to an int

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

// Note: why do the mention the E=1 direct-mapped cache? am I supposed to do something with this
// or is it just to help us understand things??

/* fn operateFlags(cache, trace file):



    */


/* fn operateCache(cache, address) -> returns nothing (but updates HIT, MISS and EVICTION counts):
    calls the fn checkCache

    if (fn checkCache(cache, address)) == HIT: update the HIT count
    if (fn checkCache(cache, address)) == MISS: update the MISS count and insert the address
    if (fn checkCache(cache, address)) == FULL: update the MISS count, update the EVICTION count and call the evict function
*/

/* fn checkCache(cache, address) -> returns a string:

if address is there: update the recency and return HIT
if address is NOT there and set has room:: (insert the address???) return MISS
if address is NOT there and set is full: return FULL
*/

/* fn evict(cache, address, policy) -> returns nothing (but modifies the cache)

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

