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
    dar 2 halat compiler be code ha negah mikone va az vojodeshon bakhabar mishe:
        1 - agar inja bashe : cargo.toml > dependencies
        2 - behesh begi hamchin file vojood darad mesle :  mod module_1;

Inline
```rust
// src/main.rs
fn main() 
{
    module_1::fn_1();
}

mod module_1
{
    pub fn fn_1() 
    {
        println!("fn_1");
    }
}
```

File-based
```rust
// src/main.rs
mod module_1;
use module_1::fn_1;

fn main() 
{
    fn_1();
}

// src/module_1.rs
pub fn fn_1() 
{
    println!("fn_1");
}
```

Grouping Related Code in Modules
```rust
// src/main.rs
fn main() 
{
    module_1::module_a::fn_a();
    module_1::module_b::fn_b();
}

mod module_1
{
    pub mod module_a
    {
        pub fn fn_a() 
        {
            println!("fn_a");
        }
    }

    pub mod module_b
    {
        pub fn fn_b() 
        {
            println!("fn_b");
        }
    }
}
```



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

Absolute
    crate:: when referencing modules from the top level, especially across files.
    self:: for local submodules or internal structure within a file.
```rust
// src/main.rs
mod util;

fn main() 
{
    crate::util::fn_util();
    self::util::fn_util();
}

// src/util.rs
pub fn fn_util() 
{
    println!("fn_util");
}
```

Relative
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



<!--------------------------------------------------------------------------------- Other -->
<br><br>

## Other

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