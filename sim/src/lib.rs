use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};

pub struct Line {
    tag: u32,
		set_index: u32,
		valid: bool,
		recency: u32,
}

pub struct Set {
	lines: Vec<Line>,
	access_counter: u32,
}

pub struct Cache {
	sets: Vec<Set>,
}

/*
impl Cache {
    // Create a new empty Cache
    pub fn new() -> Self {
        Cache { sets: Vec<Set>::new() }
    }

		required methods for cache:
		calculate 

    // Insert or update a word and its definition in the dictionary
    pub fn insert_word(&mut self, word: &str, definition: &str) {
        self.data.insert(word.to_string(), definition.to_string());
    }

    // Look up the definition of a word in the dictionary
    pub fn lookup_word(&self, word: &str) -> &str {
        match self.data.get(word) {
            Some(definition) => definition,
            None => "Word doesn't exist",
        }
    }

    // Delete a word from the dictionary
    pub fn delete_word(&mut self, word: &str) {
        self.data.remove(word);
    }
}
	let sets_sum = 2_u32.pow(set_bits);							// sets_sum == S == total sets
  let block_size = 2_u32.pow(block_bits);      		// block_size == B == total bytes per block
	let cache_size: u32 = sets_sum * lines * block_size;


*/

pub mod parse_args {

	use getopt::Opt;
	use std::error::Error;
	use std::num::ParseIntError;

	#[allow(unused)]
	pub fn parse_them(args: &Vec<String>) -> Result<(u32, u32, u32, String), Box<dyn Error>> {
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

		if h || s < 1 || e < 1 || t.is_empty() {
			print_usage_msg();
			return Err("other problem".into());
		}
		Ok((s as u32, e as u32, b as u32, t))
	}

	pub fn print_usage_msg() {
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
}