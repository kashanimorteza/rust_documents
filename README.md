<!--------------------------------------------------------------------------------- Description -->
# Rust
    this is documents of rust



<!--------------------------------------------------------------------------------- Resource -->
<br><br>

## Resource  

Web

    Web        : https://www.rust-lang.org/
    Github     : https://github.com/rust-lang
    Package    : https://crates.io/
    Document   : https://docs.rs/
    Online IDE : https://play.rust-lang.org/
    Examples   : https://doc.rust-lang.org/rust-by-example/
    WebLog     : https://blog.alihoseiny.ir/category/%d8%a2%d9%85%d9%88%d8%b2%d8%b4-%d8%b2%d8%a8%d8%a7%d9%86-%d8%a8%d8%b1%d9%86%d8%a7%d9%85%d9%87%e2%80%8c%d9%86%d9%88%db%8c%d8%b3%db%8c-rust/

Book

    Farsi      : https://rustbook.drunkleen.com/title-page.html
    English    : https://doc.rust-lang.org/stable/book/
    experiment : https://rust-book.cs.brown.edu

Video

    @drunkleen    : https://www.youtube.com/watch?v=4SE-7UM0Ny8&list=PLEQkjob0yvF-Zai14ERqrTB-lAelqTJfm
    @MrAlihoseiny : https://www.youtube.com/watch?v=oNJxb6Vfu_I&list=PLprJq5MbyJebie1Xps1Cap82gkId9EIuU


<!--------------------------------------------------------------------------------- Install -->
<br><br>

## Install 

Mac
```bash
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
brew install rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
echo $HOME/.cargo/env >> $HOME/.zshrc
```

Linux
```bash
sudo apt update && sudo apt upgrade -y
sudo apt install build-essential pkg-config libssl-dev -y
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustc --version
cargo --version
rustup update
```

Windows
```bash
```

IDE
```bash
Vs-Code
Rust Extension Pack
```



<!--------------------------------------------------------------------------------- Tools -->
<br><br>

## Tools
    rustup
    cargo
    rust-analyzer
    rust-gdb
    rust-gdbgui
    rust-lldb
    rustc
    rustdoc
    rustfmt



<!--------------------------------------------------------------------------------- Roadmap -->
<br><br>

## Roadmap
    Base 10 | Base 2
    Bit | Byte | Store integer, char, string on memory
    Stack And Heap | Garbage collection
    Scop | Prelude
    macro



<!--------------------------------------------------------------------------------- Type -->
<br><br>

## Type
Scalar
    
    Integer
    Float
    Boolean
    Char

Compound

    Tuple
    Array
    Custom : Structs, Enums
    String
    Slice
    Collection
    Pointer and Reference
    Miscellaneous



<!--------------------------------------------------------------------------------- Variable -->
<br><br>

## Variable
    Declar
    Mutability
    Const
    Shadowing
    Environment
    
    Vector
    Refrence
    DeRefrence
    Pointer
    Smart poiner
    



<!--------------------------------------------------------------------------------- Operators and Symbols -->
<br><br>

## Operators
    Operators
    Stand-Alone Syntax
    Path-Related Syntax
    Generics
    Trait Bound Constraints
    Macros and Attributes
    Comment
    Parentheses
    Square Brackets



<!--------------------------------------------------------------------------------- Condition -->
<br><br>

## Condition
    If
    Match
    Let if



<!--------------------------------------------------------------------------------- Loop -->
<br><br>

## Loop
    Loop
    For
    While



<!--------------------------------------------------------------------------------- Function -->
<br><br>

## Function
    Function
    Closure
    Iterators
    Associated Function



<!--------------------------------------------------------------------------------- Crate -->
<br><br>

## Modularity
    Workspaces :
    Packages   : binary, library
    Modules    :
    Crates     :
    Path       : absolute, relative, pub, supper, use, pub use, glob, as, External Packages



<!--------------------------------------------------------------------------------- Ownership -->
<br><br>

## Ownership
    RC : Reference Counted smart pointer



<!--------------------------------------------------------------------------------- Borrowing -->
<br><br>

## Borrowing



<!--------------------------------------------------------------------------------- LifeTime -->
<br><br>

## LifeTime



<!--------------------------------------------------------------------------------- Smart pointer -->
<br><br>

## Smart pointer
    Box
    Deref
    Wrapper
    Dynamic box 



<!--------------------------------------------------------------------------------- Generic -->
<br><br>

## Generic



<!--------------------------------------------------------------------------------- Trait -->
<br><br>

## Trait



<!--------------------------------------------------------------------------------- Concurrency -->
<br><br>

## Concurrency
    Process
    Threads : Spawn, Move
    Channel
    Shared-State
    Mutex
    Race conditions | Deadlocks



<!--------------------------------------------------------------------------------- Asynchronous -->
<br><br>

## Asynchronous
    Futures



<!--------------------------------------------------------------------------------- Error Handling -->
<br><br>

##  Error Handling
    panic
    abort
    unwrap
    expect
    Result
    Multiple error types
    Iterating over Results





<!--------------------------------------------------------------------------------- Resource -->
<br><br>

## Resource



<!--------------------------------------------------------------------------------- Attributes -->
<br><br>

## Attributes
    Derive
    Conditional Compilation
    Lint Control
    Macro-related
    Crate-level






<!--------------------------------------------------------------------------------- OOP -->
<br><br>

## OOP
    Objects
    Encapsulation
    Inheritance
    Polymorphism



<!--------------------------------------------------------------------------------- ORM -->
<br><br>

## ORM



<!--------------------------------------------------------------------------------- API -->
<br><br>

## API

[axum]



<!--------------------------------------------------------------------------------- Pub/Sub -->
<br><br>

## Pub/Sub



<!--------------------------------------------------------------------------------- Test -->
<br><br>

## Test



<!--------------------------------------------------------------------------------- Concept -->
<br><br>

## Concept
    Unwinding
    Buffer overread



<!--------------------------------------------------------------------------------- Crates -->
<br><br>

## Crates

Tokio

Tower



<!--------------------------------------------------------------------------------- Note -->
<br><br>

## Note 
    Life time | Segmentation fault | Raw Identifiers
    1 - Write a libreary and import to main source


[axum]: https://github.com/kashanimorteza/rust_documents/tree/main/api_axum.md
