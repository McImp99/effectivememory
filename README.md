Time complexity between Rust and Java solutions are linear O(n). Rust is roughly 9 times faster in a cargo run --release than the java equivalent due to better cache locality, fewer heap allocations and no garbage collection

Language	n	    Creation runtime
Rust	1,000,000	3 ms
Rust	100,000	    0 ms
Java	1,000,000	18 ms
Java	100,000	    10 ms


For hashmap test:
Rust
Lookup time: 500 ns
Total build time: 19 ms

Java:
Build time: 37 ms
Lookup time: 18200 ns




