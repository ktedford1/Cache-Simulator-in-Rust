pub mod parse_args {

	use getopt::Opt;
	use std::error::Error;

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


	pub fn process_address(binary_value: &u64, block_bits: &u64, set_bits:&u64) -> (u64, u64) {
		let tag_and_set = binary_value >> block_bits;
		let sets = 2_u32.pow(*set_bits as u32);	
		let sets = sets as u64;
		let tag = tag_and_set / sets;														// store the quotient as tag
		let set_index = tag_and_set % sets;											// store the remainder as set_index
		println!("tag: {} set: {}", tag, set_index);
		(tag, set_index)
	}

}