// import std:: functions

pub fn main() {

// collect command line arguments

// validate command line arguments

// parse command line arguments

// store the command line arguments in variables

// further program logic

}

// create struct for 'Line'
// create struct for 'Set'
// create struct for 'Cache' -- separate files?

// pub fn checkCache(address: &str, cache: ??) -> returns a bool to answer, is addresss in cache?Y/N
// if the address is in the cache -> 
//    update the line's recency info and return an enum value "HIT"
// if the address is not found:
//    empty spot in the set? 
//      then insert the address into the cache and return an enum value "MISS"
//    no empty spot in the set?
//      return an enum value "FULL" to indicate a MISS

// pub fn evict(address, cache, policy) -> returns nothing
// evict one based on FIFO or LRU policy
// update the evicted line's tag, recency info, and placement info

// pub fn operateCache(address, cache) -> returns nothing
// uses the fn checkCache to determine if an address is in the cache
// if the address is in the cache -> 
//    increment the HIT count
// if the address is not found:
//    empty spot in the set? 
//      then insert the address into the cache and increment the MISS count
//    no empty spot in the set?
//      call the evict function
//      increment the EVICT count
//      increment the MISS count 

// pub fn operateFlags(trace file, cache) -> returns nothing
// reads the trace file line-by-line and calls the operateCache function for each memory aces
// prints out the final statistics for the cache, either succinct or verbose

// create a function to translate hexadecimal addresses to binary/decimal??
