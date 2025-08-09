<!--------------------------------------------------------------------------------- Description -->
# Rust
    this is documents of rust



<!--------------------------------------------------------------------------------- Resource -->
<br><br>

## Resource  
    Web        : https://www.rust-lang.org/
    Github     : https://github.com/rust-lang
    Package    : https://crates.io/
    Document   : https://docs.rs/
    Online IDE : https://play.rust-lang.org/
    Book       : https://rustbook.drunkleen.com/title-page.html
    Examples   : https://doc.rust-lang.org/rust-by-example/
    Video      : @drunkleen    : https://www.youtube.com/watch?v=4SE-7UM0Ny8&list=PLEQkjob0yvF-Zai14ERqrTB-lAelqTJfm
    Video      : @MrAlihoseiny : https://www.youtube.com/watch?v=oNJxb6Vfu_I&list=PLprJq5MbyJebie1Xps1Cap82gkId9EIuU
    WebLog     : https://blog.alihoseiny.ir/category/%d8%a2%d9%85%d9%88%d8%b2%d8%b4-%d8%b2%d8%a8%d8%a7%d9%86-%d8%a8%d8%b1%d9%86%d8%a7%d9%85%d9%87%e2%80%8c%d9%86%d9%88%db%8c%d8%b3%db%8c-rust/



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



<!--------------------------------------------------------------------------------- Concept -->
<br><br>

## Concept
    Unwinding
    Buffer overread



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
    String
    Collection
    Custom
    Pointer and Reference
    Miscellaneous



<!--------------------------------------------------------------------------------- Variable -->
<br><br>

## Variable
    Declar
    Mutability : Immutable, Mutable
    Const
    Shadowing
    Array
    Vector
    Refrence
    DeRefrence
    Life time
    Pointer
    Smart poiner
    Environment



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
    Function | Parameters | Return
    Associated Function
    Generic
    Traits
    Closure
    Iterators



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




<!--------------------------------------------------------------------------------- Borrowing -->
<br><br>

## Borrowing



<!--------------------------------------------------------------------------------- Concurrency -->
<br><br>

## Concurrency
    Race conditions | Deadlocks
    Process
    Threads
    Spawn
    Move
    Channel
    Shared-State
    Mutex



<!--------------------------------------------------------------------------------- Asynchronous -->
<br><br>

## Asynchronous
    Futures



<!--------------------------------------------------------------------------------- Error -->
<br><br>

## Error
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



<!--------------------------------------------------------------------------------- Extra -->
<br><br>

## Extra
    ORM
    API



<!--------------------------------------------------------------------------------- Note -->
<br><br>

## Note 
    Life time | Segmentation fault | Raw Identifiers
    1 - Write a libreary and import to main source