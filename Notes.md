# Notes on Rust

## Chapter 2

- std = standard library.
- prelude is the types Rust brings into every program
  - Everything outside this scope needs to be imported with the keyword `use`
  - std::io is one such library

```rust
let foo = 5; // immutable
let mut bar = 5; // mutable
```

- variables are immutable by default
- comments written with // prefix
- `::` in `String::new()` indicates that the new() is an associated function of the type `String`.
- `&` indicates the argument is a reference.
- `Result` is an Enum with the value Ok or Err.
  - Allows us to use `.expect` for error handling
- `match` is pretty cool.
  - Allows us to put in multiple "arms" and make the program check each arm for a match.
  - Each arm can have a way to handle the program. So one arm can break the loop while another can print it.
