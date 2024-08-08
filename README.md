# Rust_Learning

----- Ownership rules -----
1. Each value in Rush has a variable that's called its owner.
2. There can only be one owner at a time.
3. When the owner goes out of the scope, the value will be dropped.

----- The Rules of References -----
1. At any given time, you can have either one mutable reference or any number of immutable references
2. References must alway be valid.
