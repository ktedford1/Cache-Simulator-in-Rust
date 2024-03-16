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