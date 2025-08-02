
# Borrowing
    All example about Borrowing



<!--------------------------------------------------------------------------------- Description -->
<br><br>

## Description
    We call the action of creating a reference borrowing

    The Rules:
    -----------
    At any given time, you can have either one mutable reference or any number of immutable references.
    References must always be valid.

    Data race:
    -----------
    Two or more pointers access the same data at the same time.
    At least one of the pointers is being used to write to the data.
    There’s no mechanism being used to synchronize access to the data.



<!--------------------------------------------------------------------------------- Immutable -->
<br><br>

## Immutable
```rust
fn main() 
{
    let s = String::from("hello");
    let l = calculate_length(&s);
    println!("s: {s}   l: {l}");
}

fn calculate_length(s: &String) -> usize
{
    s.len()
}
```

We can borrow s more than once at a time
```rust
fn main() 
{
    let s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    let r3 = &s;
    
    println!("{s}, {r1}, {r2}, {r3}");
}
```



<!--------------------------------------------------------------------------------- Mutable -->
<br><br>

## Mutable
```rust
fn main() 
{
    let mut s = String::from("hello");
    change(&mut s);
    println!("s: {s}");
}

fn change(s: &mut String) 
{
    s.push_str(", world");
}
```

We cannot borrow s as mutable more than once at a time
```rust
fn main() 
{
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;
    
    println!("{}, {}", r1, r2);
}
```

First r1 must used and allocate r2
```rust
fn main() 
{
    let mut s = String::from("hello");

    let r1 = &mut s;
    println!("r1: {r1}");

    let r2 = &mut s;
    println!("r2: {r2}");
}
```

We also cannot have a mutable reference while we have an immutable one to the same value
```rust
fn main() 
{
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);
}
```

First r1 and r2 must used and allocate r3
```rust
fn main() 
{
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{r1} and {r2}");

    let r3 = &mut s;
    println!("{r3}");
}
```



<!--------------------------------------------------------------------------------- Dangling -->
<br><br>

## Dangling
Create a dangling reference
```rust
fn main() 
{
    let reference_to_nothing = dangle();
}

fn dangle() -> &String 
{
    let s = String::from("hello");
    &s
}
```