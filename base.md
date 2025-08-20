
# Bse
    All example about Base



<!--------------------------------------------------------------------------------- Scop -->
<br><br>

## Scop
```rust
fn main() 
{
    {                      
        // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward
        // do stuff with s
    }
    // this scope is now over, and s is no longer valid
}
```



<!--------------------------------------------------------------------------------- Prelude -->
<br><br>

## Prelude
    The Rust prelude is a small collection of common types, traits, 
    and functions that the compiler automatically brings into scope in every file.

Disable the Prelude
```rust
#![no_implicit_prelude]
fn main() 
{
    
}
```



<!--------------------------------------------------------------------------------- Stack And Heap -->
<br><br>

## Stack And Heap
    Both the stack and the heap are parts of memory available to your code to use at runtime, but they are structured in different ways

    All data stored on the stack must have a known, fixed size
    No Allocated
    Pushing values
    Pushing to the stack is faster than allocating on the heap

    Data with an unknown size at compile time or a size that might change must be stored on the heap
    Allocated
    Pushing values
    Return pointer and store on stack
    The memory must be requested from the memory allocator at runtime.
    We need a way of returning this memory to the allocator when we’re done with our String.

Store on Stack
```rust
fn main() 
{
    let s = "hello";
    println!("The value of x is: {s}");
}
```

Store on Heap
```rust
fn main() 
{
    let mut s = String::from("hello");
    s.push_str(", world!"); 
    println!("{s}");
}
```