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


## Chapter 6

- Enums are still enums in rust
```rust
// define values
let four = IpAddrKind::V4;
// use enum type
fn route(ip_kind: IpAddrKind) {}
```

- null er bad
- match er ganske sick
```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

- `_`is anything
- if let takes in a pattern and an expression. 

```rust
let some_u8_value = Some(0u8);
if let Some(3) = some_u8_value {
    println!("three");
}
```

## Chapter 7
- Rusts module system includes
  - packages
    - build test and share crates
  - crates
    - tree of modules that produce a library/executable
  - modules and use
    - control organization, scope and privacy of paths
  - paths
    - a way of naming an item, such as struct, function or module
- a crate groups related functionality together in a scope.
  - easy to share between projects.

```rust

use std::io::{self, Write};
```
- Brigns in std::io and std::io::Write into the scope.
```rust

use std::collections::*;
```
- brings everythng is.

## Chapter 8
- Vectors are basically a list which can expand
```rust
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
```

- you get eleents from a vector with squarebracket and its index

- using get on a vector will allow it to keep running. Accessing it with squarebracket will cause the program to panic


- strings are a collection of characters 

- HashMaps uses `insert`. where you write the the key then value
- its possible to zip two lsits into a hashmap where you pair them based on index

- You can access with get(key)
```rust
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
```
- loop through a hashmap
- you can overwrite a value by using the same key when inserting
```rust
   scores.entry(String::from("Yellow")).or_insert(50);
```

- insert if key has no value
- 