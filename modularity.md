# Modularity
    Description : All example about Modularity
    List        : io | round


<!--------------------------------------------------------------------------------- Workspace -->
<br><br>

## Workspace



<!--------------------------------------------------------------------------------- Package -->
<br><br>

## Package
    A package is a bundle of one or more crates that provides a set of functionality
    A package contains a Cargo.toml file that describes how to build those crates
    Cargo is actually a package that contains the binary crate for the command line tool you’ve been using to build your code



<!--------------------------------------------------------------------------------- Module -->
<br><br>

## Module



<!--------------------------------------------------------------------------------- Crate -->
<br><br>

## Crate
    A crate is the smallest amount of code that the Rust compiler considers at a time
    A crate can come in one of two forms: a binary crate or a library crate

### Binary
    Each must have a function called main that defines what happens when the executable runs

### Library
    Library crates don’t have a main function, and they don’t compile to an executable. Instead, they define functionality intended to be shared with multiple projects






<!--------------------------------------------------------------------------------- Release -->
<br><br>

## Paths
### absolute
```rust
    crate::front_of_house::hosting::add_to_waitlist();
```

### relative
```rust
    front_of_house::hosting::add_to_waitlist();
```

### pub
```rust
```

### supper
```rust
```

### use
```rust
use std::io;
use std::io::{self, Write};
```

### pub use
```rust
```

### glob
```rust
use std::collections::*;
```

### as
```rust

```

### external Packages
```rust
use rand::Rng;

fn main() 
{
    let rand_number = rand::rng().random_range(1..=999);
    println!("rand_number: {}", rand_number);
}
```