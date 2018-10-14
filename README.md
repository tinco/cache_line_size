Cache Line Size
============

This is a crate that gives access to the cache line size of a given architecture. It also has a generic type that can be used to align
its parameter to the cache line size.

For example, to have a struct with three `u8` with each on its own
cache line, you could write the following code:

```rust

use cache_line_size::{CacheAligned, CACHE_LINE_SIZE};
use std::mem::size_of;

struct ThreeLineStruct {
  line_1: CacheAligned<u8>,
  line_2: CacheAligned<u8>,
  line_3: CacheAligned<u8>,
}

#[test]
fn it_is_three_lines() {
    assert_eq!(size_of::<ThreeLineStruct>(), 3*CACHE_LINE_SIZE);
}

```
