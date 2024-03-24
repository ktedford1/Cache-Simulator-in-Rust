pub mod parse_args {

	use getopt::Opt;
	use std::error::Error;

	// code citation: code for 'fn parse_them' was essentially copied from https://docs.rs/getopt/latest/getopt/struct.Parser.html
	// and https://crates.io/crates/getopt and slightly modified to fit the expected arguments for the cache sim

	#[allow(unused)]
	pub fn parse_them(args: &Vec<String>) -> Result<(usize, usize, usize, String), Box<dyn Error>> {
		let mut opts = getopt::Parser::new(&args, "hvs:E:b:t:");
		let mut h = false;						// help flag
		let mut v = false;						// verbose flag	- Note: the verbose function is not set up yet
		let mut s = String::new();		// set bits
		let mut e = String::new();		// total lines per set
		let mut b = String::new();		// block bits
		let mut t = String::new();		// filepath to text file of Valgrind memory trace, e.g. "/home/codio/workspace/traces/yi.trace"

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

		// to rule out negative numbers as arguments, 's', 'e', and 'b' have to be signed integers, so temporarily convert them to 'i32'
		let s: i32 = s.parse()?;			// convert the arg 's' from a string to an int
		let e: i32 = e.parse()?;			// convert the arg 'e' from a string to an int
		let b: i32 = b.parse()?;			// convert the arg 'b' from a string to an int

		// if help is requested, or if the number of sets or lines is less than 1, or if there is no textfile, print the usage msg
		if h || s < 1 || e < 1 || t.is_empty() {
			print_usage_msg();
		}

		Ok((s as usize, e as usize, b as usize, t))
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
