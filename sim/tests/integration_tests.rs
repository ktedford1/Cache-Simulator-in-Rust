use sim::parse_args::parse_them;
use sim::read_file_by_line::read_file_by_line;
use sim::Cache;
use sim::read_file_by_line::process_address;

#[test]
fn test_parse_them() {
  // test CLI parsing with valid arguments
  let args: Vec<String> = vec!["target/debug/sim".to_string(), "-s".to_string(), "4".to_string(), "-E".to_string(), "1".to_string(), "-b".to_string(), "4".to_string(), "-t".to_string(), "/home/codio/workspace/traces/yi.trace".to_string()];
	let result = parse_them(&args);
	assert!(result.is_ok());
	let (s, e, b, t) = result.unwrap();
	assert_eq!((s, e, b, t), (4 as usize, 1 as usize, 4 as usize, "/home/codio/workspace/traces/yi.trace".to_string()));

	// test CLI parsing with invalid arguments (missing 's') - changed code to print out usage message instead
	/*let args: Vec<String> = vec!["target/debug/sim".to_string(), "-E".to_string(), "1".to_string(), "-b".to_string(), "4".to_string(), "-t".to_string(), "/home/codio/workspace/traces/yi.trace".to_string()];
	let result = parse_them(&args);
	assert!(result.is_err());*/
}

#[test]
fn test_read_file_by_line() {
	// test results with valid input: test values s = 2, E = 1, b = 4, t = traces/yi.trace
	let set_bits = 2;
	let block_bits = 4;
	let filepath = "/home/codio/workspace/traces/yi.trace";
	let mut test_cache = Cache::new_cache(4, 1);
	assert!(read_file_by_line(&mut test_cache, filepath, &block_bits, &set_bits).is_ok());
	assert_eq!(test_cache.hits, 4);
	assert_eq!(test_cache.misses, 5);
	assert_eq!(test_cache.evictions, 3);
}

#[test]
fn test_read_file_by_line_bad_filepath() {
	// test results with invalid filepath
	let set_bits = 2;
	let block_bits = 4;
	let filepath = "/home/codio/workspace/traces/doesnt_exist.trace";
	let mut test_cache2 = Cache::new_cache(4, 1);
	assert!(read_file_by_line(&mut test_cache2, filepath, &block_bits, &set_bits).is_err());
}

#[test]
fn test_read_file_by_line_bad_hex_address() {
	// test results with invalid hex_address
	let set_bits = 2;
	let block_bits = 4;
	let filepath = "/home/codio/workspace/traces/error_bad_hex_yi.trace";
	let mut test_cache3 = Cache::new_cache(4, 1);
	let result = read_file_by_line(&mut test_cache3, filepath, &block_bits, &set_bits);
	assert!(result.is_err());
}

#[test]
fn test_read_file_by_line_bad_operation() {
	// test results with invalid operation
	let set_bits = 2;
	let block_bits = 4;
	let filepath = "/home/codio/workspace/traces/error_bad_operation_yi.trace";
	let mut test_cache3 = Cache::new_cache(4, 1);
	let result = read_file_by_line(&mut test_cache3, filepath, &block_bits, &set_bits);
	assert!(result.is_err());
}

#[test]
fn test_process_address_valid_input() {
	// test results with valid input
	let binary_value = 0xaabbccdd_u64;
	let block_bits = 8;
	let set_bits = 8;

	let (tag, set_index) = process_address(&binary_value, &block_bits, &set_bits);
	assert_eq!(tag, 0xaabb);
	assert_eq!(set_index, 0xcc);
}

#[test]
fn test_process_address_no_tag() {
	// test results with a hex address that has no tag
	let binary_value = 0xaabbccdd_u64;
	let block_bits = 16;
	let set_bits = 16;

	let (tag, set_index) = process_address(&binary_value, &block_bits, &set_bits);
	assert_eq!(tag, 0x0);
	assert_eq!(set_index, 0xaabb);
}

#[test]
// test new_cache function with valid input
fn test_new_cache() {
	let tester = Cache::new_cache(2, 3);
	assert_eq!(tester.sets.len(), 2);
	assert_eq!(tester.sets[0].lines.len(), 3);
	assert_eq!(tester.sets[0].lines[1].recency, 0);
}	
