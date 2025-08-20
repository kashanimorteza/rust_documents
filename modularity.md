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

Grouping : Inline
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

Grouping : File-based
```rust
// src/main.rs
mod util;
fn main() 
{
    util::module_1::a::fn_1();
    util::module_1::b::fn_1();
    util::module_2::a::fn_1();
    util::module_2::b::fn_1();
}

// src/util/mod.rs
pub mod module_1;
pub mod module_2;

// src/module_1/a.rs
pub fn fn_1() 
{
    println!("a");
}

// src/module_1/b.rs
pub fn fn_1() 
{
    println!("b");
}

// src/module_1/mod.rs
pub mod a;
pub mod b;

// src/module_2/a.rs
pub fn fn_1() 
{
    println!("a");
}

// src/module_2/b.rs
pub fn fn_1() 
{
    println!("b");
}

// src/module_2/mod.rs
pub mod a;
pub mod b;
```

Super
```rust
// src/util/mod.rs
mod util;

fn main() 
{
    util::math::call_parent();
}

// src/util/mod.rs
pub mod math;

pub fn shared_fn() 
{
    println!("Called from shared_fn in utils");
}

// src/util/math.rs
pub fn call_parent() 
{
    // `super` refers to utils module (mod.rs)
    super::shared_fn();
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

### Absolute
    crate:: When referencing modules from the top level, especially across files.
    self::  For local submodules or internal structure within a file.

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



### Relative
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


### pub
    Baraye public kardan estefade mishe 

    pub mod mudule_1 : 
        az in khat be bad dige compiler dar kole proje emkane negah be in file ro dare
        agar pub nadashte bashe compiler faghat hamon mahdodeii ke in khat hast be on file negah mikone

    pub use std::io :
        az in ja be bad dige access hast dar kole proje
        agar pub nadashte bashe faghat dar hamon mahdode access hast



### use
```rust
use std::io;
use std::io::{self, Write};
use std::{cmp::Ordering, io};
use std::collections::*;
pub use std::io;
```