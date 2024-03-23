// Import the dictionary library
use sim::Cache;

#[test]
fn test_insert_word() {
  // Instantiate the Dictionary and assign it to a mutable variable called dictionary
  let mut dictionary = Dictionary::new();
  
  // Insert the word "rust" with definition "A programming language" into dictionary
  dictionary.insert_word("rust", "A programming language");

  assert_eq!(dictionary.lookup_word("rust"), "A programming language");
  
  // Update the definition for the existing word "rust" with the definition "Updated definition" in the dictionary
  dictionary.insert_word("rust", "Updated definition");

  assert_eq!(dictionary.lookup_word("rust"), "Updated definition");
}

#[test]
fn test_lookup_word() {
  // Instantiate the Dictionary and assign it to a mutable variable called dictionary
  let mut dictionary = Dictionary::new();
  
  // Insert the word "rust" with definition "A programming language" into dictionary
  dictionary.insert_word("rust", "A programming language");
  
  // Insert the word "dictionary" with definition "A collection of words and their meanings" into `dictionary`
  dictionary.insert_word("dictionary", "A collection of words and their meanings");

  assert_eq!(dictionary.lookup_word("rust"), "A programming language");
  assert_eq!(dictionary.lookup_word("dictionary"), "A collection of words and their meanings");
  assert_eq!(dictionary.lookup_word("nonexistent"), "Word doesn't exist");
}

#[test]
fn test_delete_word() {
  // Instantiate the Dictionary and assign it to a mutable variable called dictionary
  let mut dictionary = Dictionary::new();
  
  // Insert the word "rust" with definition "A programming language" into dictionary
  dictionary.insert_word("rust", "A programming language");
  
  // Insert the word "dictionary" with definition "A collection of words and their meanings" into `dictionary`
  dictionary.insert_word("dictionary", "A collection of words and their meanings");
  assert_eq!(dictionary.lookup_word("rust"), "A programming language");

  // Delete the word "rust" from the Dictionary
  dictionary.delete_word("rust");
  assert_eq!(dictionary.lookup_word("rust"), "Word doesn't exist");
}