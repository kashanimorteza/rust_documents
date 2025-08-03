# Modularity
    Description : All example about Modularity
    List        : io | round



<!--------------------------------------------------------------------------------- Crate -->
<br><br>

## Crate
    A crate is the smallest amount of code that the Rust compiler considers at a time
    A crate can come in one of two forms: a binary crate or a library crate

Binary

    Each must have a function called main that defines what happens when the executable runs

Library

    Library crates don’t have a main function, and they don’t compile to an executable. Instead, they define functionality intended to be shared with multiple projects


<!--------------------------------------------------------------------------------- Module -->
<br><br>

## Module



<!--------------------------------------------------------------------------------- Package -->
<br><br>

## Package
    A package is a bundle of one or more crates that provides a set of functionality
    A package contains a Cargo.toml file that describes how to build those crates
    Cargo is actually a package that contains the binary crate for the command line tool you’ve been using to build your code



<!--------------------------------------------------------------------------------- Workspace -->
<br><br>

## Workspace



<!--------------------------------------------------------------------------------- Paths -->
<br><br>

## Paths

### absolute
```rust
// src/main.rs
mod util;

fn main() 
{
    util::fn_util();
}

// src/util.rs
pub fn fn_util() 
{
    println!("fn_util");
}
```

### relative
```rust
// src/main.rs
mod util;

fn main() 
{
    util::fn_util();
}

// src/util.rs
pub fn fn_util() 
{
    println!("fn_util");
}
```

### mod
    dar 2 halat compiler be code ha negah mikone va az vojodeshon bakhabar mishe:
        1 - cargo.toml > dependencies
        2 - behesh begi hamchin file vojood darad mesle > mod util;

    mod util : faghat dar hamon file ghabele didan hast
    pub mod util : to kole file ha va project ghabele didan hast

```rust
// src/main.rs
mod util;

fn main() 
{
    util::fn_util();
}

// src/util.rs
pub fn fn_util() 
{
    println!("fn_util");
}
```

```rust
// src/main.rs
pub mod util;
pub mod tools;

fn main() 
{
    util::fn_util();
}

// src/util.rs
pub fn fn_util() 
{
    crate::tools::fn_tools();
}

// src/tools.rs
pub fn fn_tools() 
{
    println!("fn_tools");
}
```

### use
```rust
use std::io;
use std::io::{self, Write};
```

### pub
```rust
```

### supper
```rust
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