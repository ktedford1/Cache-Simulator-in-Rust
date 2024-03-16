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
	// copied code for this!!
	
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
	// should I put all this parsing stuff in a function? if so, how do I return the variables / transfer ownership?
	//  --> check 'let's get Rusty'
	// do I have to use "E" everywhere as in the course specs? should I turn off snake_case? Or does it not matter, 
	// since only output is scored?
	// put in a validation for the datatypes of the variables, e.g. no negative numbers! no zero! or?? can b = 0??
	// is there a way to validate the text file?
	// add more comments

	// don't forget to deal with verbose case!! see Lab exercise from Topic 4 or 6

  if h || s.is_empty() || e.is_empty() || b.is_empty()|| t.is_empty() {
    print_usage_msg();
		return Ok(());
  }

	let set_bits: u32 = s.parse()?;								// convert the arg 's' from a string to an int and store it in 'set_bits'
	let sets_sum = 2_u32.pow(set_bits);      			// S == total sets
	let block_bits: u32 = b.parse()?;							// convert the arg 'b' from a string to an int and store it in 'block_bits'
  let block_size = 2_u32.pow(block_bits);      // B == total words per block
  let lines: u32 = e.parse()?;								// convert the arg 'e' from a string to an int and store it in 'lines'
	let cache_size: u32 = sets_sum * block_size * lines;

	// do I have to add a trim function to get rid of white spaces?

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
// I think these structs are nested: how to write this??
/*
SEPARATE FILE (like lib.rs from topic 8 lab):

pub struct Cache {
  sets: [Set, S],             // can I use an expression for the size of the array?
  cache_parameters: u32,        // which parameters?
  performance_stats: u32,       // which stats?
  eviction_policy_flag: bool    // i.e. LRU or FIFO, apparently we only need to do LRU??

struct Set {
  lines: [Line, E],       // can I use a variable for the number of items in an array? 
  current_rate: u32,      // what does this mean?
  placement_rate: u32     // what does this mean?

struct Line {
  block: u32,
  validity: bool,         // check this
  tag: u32,
  recency: u32

}
*/

// Note: why do the mention the E=1 direct-mapped cache? am I supposed to do something with this
// or is it just to help us understand things??

/* user gives you 's', 'E', 'b', and a trace file
  you calculate total # available lines in cache: 
      1 line holds 1 block 'B' containing 2^b bytes per line
      Therefore: E lines per set * S (2^s) sets == total # of blocks
			Therefore: E lines per set * S (2^s) sets * B (2^b) bytes == total # of bytes
  you initialize the 'Set' and 'Cache' arrays with the values of 'E' and 'S'

how to get the set index and tag:
	trace file gives you an address in hexadecimal
	convert it to binary, call it "x", then right-shift "x" by "b" bits (discard the block offset)
	apparently, number modulo max value of a number of bits returns those bits
	citation: https://de.mathworks.com/matlabcentral/answers/625063-how-to-divide-8-bit-binary-into-two-4-bit
	(see answer from Xavier)
	then calculate: 	x % S to get the set index (the new least significant 's' bits)
	then right-shift "x" again by 's' bits to get rid of them, and the remaining number is the tag index

then initialize a struct for Cache with this new address:
    in struct Cache: add a set index 's'
    in struct Set (inside Cache):
				add a line:
					add a block size (2^b)
					switch validity tag to 1  (??should this be an enum? or a bool?)
					add a tag index
					add a date stamp

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

