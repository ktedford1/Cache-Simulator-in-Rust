use std::env;
// use std::fs::File;
// use std::io::{self, BufRead};
use getopt::Opt;
// use getopt::Parser;
use std::error;
#[allow(unused)]

fn main() -> Result<(), Box<dyn error::Error>> {

  // code citation for getopt parsing: https://docs.rs/getopt/latest/getopt/struct.Parser.html
	// and also https://crates.io/crates/getopt
	
  let args: Vec<String> = env::args().collect();
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
        _ => unreachable!(),
      },
    }
  }

  if h {
    print_usage_msg();
  }

	let set_bits: u32 = s.parse()?;
	let sets_sum = 2_u32.pow(set_bits);      // S == total sets
	let block_bits: u32 = b.parse()?;
  let block_size = 2_u32.pow(block_bits);      // B == total words per block
  let lines: u32 = e.parse()?;
	let cache_size: u32 = sets_sum * block_size * lines;

  println!("number of set_bits is: {}   number of sets is: {}", set_bits, sets_sum);  
	println!("number of block_bits is: {}   block size is: {}", block_bits, block_size );
  println!("cache size in words is: {}", cache_size);
	println!("text file is: {}", t);

	Ok(())

  // call fn operateFlags(cache, trace file)
  // printsln!(“Hits: {0} Misses: {1} Evictions: {2}, hits, misses, evictions);

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



// note: determine how big the block, tag, and age fields have to be
/*
struct Line {
  block: u32,
  validity: bool,         // check this
  tag: u32,
  recency: u32
}

struct Set {
  lines: [Line, E],       // can I use a variable for the number of items in an array? can I use struct 'Line' as a datatype?
  current_rate: u32,      // what does this mean?
  placement_rate: u32     // what does this mean?
}

struct Cache {
  sets: [Set, S],             // can I use an expression for the size of the array?
  cache_parameters: u32,        // which parameters?
  performance_stats: u32,       // which stats?
  eviction_policy_flag: bool    // i.e. LRU or FIFO, apparently we only need to do LRU??
}
*/

// Note: review direct-mapped cache --> relevance to E=1? does it change the logic?
// Note: am I also supposed to use 'getopt' to parse trace file data?

/* user gives you 's', 'E', 'b', and a trace file
  you calculate total # available lines in cache: 
      1 line holds 1 block 'B' containing 2^b bytes per line
      so we need to know: E lines per set * S (2^s) sets
  you also need to know how many sets there are (2^s)
  you initialize the 'Set' and 'Cache' arrays with the values of 'E' and 'S'
 trace file gives you an address 
 you get 'b' and calculate block size 'B'
 you calculate the tag 'tag' and the set index 's' and initialize a struct for Line with this new address:
    it has a set index 's' that goes in struct 'Cache'
    it goes in whatever line is empty??
    you update the validity tag of that line
    you update the recency tag of that line (how??? is this a date stamp??)
 */

/* fn operateFlags(cache, trace file):

    "The function outputs various cache statistics at the end of the simulation." --> ???
    Read the trace file line by line
    use ‘getopt’ to parse the lines and get only the lines that matter

    parse a line: 
      get the address of a word; it’s hexadecimal; Rust command to convert it to binary
      extract “tag” and “set index” according to “b” and “s”
        drop 'b' bits from the right end
        set_index = left-shift 's' bits
        tag = drop 's' bits, remainder is 't' bits
    calculate the total capacity: S * E * B bytes

      get the operation:
        ignore operation “I” lines
        load data == call fn operateCache once
        store data == call fn operateCache once
        modify data == call fn operateCache twice ????

    call the fn operateCache(cache, address)

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

