
# Ownership
    Ownership is a set of rules that govern how a Rust program manages memory

    In a systems programming language like Rust:
    whether a value is on the stack or the heap affects how the language behaves and why you have to make certain decisions

    Each value has an owner(variable).
    Each value has an owner(variable) at the time.
    When the owner(variable) goes out of scope, the value will be dropped.

    When a variable goes out of scope, Rust automatically calls the drop function and cleans up the heap memory
    Any group of simple scalar values can implement Copy
    Nothing that requires allocation or is some form of resource can implement Copy
    The mechanics of passing a value to a function are similar to those when assigning a value to a variable



<!--------------------------------------------------------------------------------- Slice -->
<br><br>

## Memory and Allocation

    When a variable goes out of scope, Rust calls a special function for us. 
    This function is called drop, and it’s where the author of String can put the code to return the memory. 
    Rust calls drop automatically at the closing curly bracket

```rust
fn main() 
{
    {
        // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward
        // do stuff with s
        println!("The value of x is: {s}");
    }
    // this scope is now over, and s is no longer valid
}
```



<!--------------------------------------------------------------------------------- Slice -->
<br><br>

## Variables 
Variables and Data Interacting with Move : Stack
```rust
fn main() 
{
    let mut x = 5;
    println!("x {}", x);

    let y = x;
    println!("y {}", y);

    //az x ye copy migirad baraye y
    x = 6;
    println!("x {}", x);
    println!("y {}", y);
}
```

Variables and Data Interacting with Move : Heap
```rust
fn main() 
{
    let s1 = String::from("hello");
    println!("s1 {}", s1);

    let s2 = s1;
    println!("s2 {}", s2);

    // s1 disable mishavad ,  engar s1 ro be s2 move mikonad
    //println!("s1 {}", s1);
}
```

Variables and Data Interacting with Clone
```rust
fn main() 
{
    let s1 = String::from("hello");
    println!("s1 {}", s1);

    let s2 = s1.clone();
    println!("s2 {}", s2);

    // Vaghan heap ro copy mikone 
    println!("s1 {}", s1);
}
```



<!--------------------------------------------------------------------------------- Slice -->
<br><br>

## Assignment
```rust
fn main() 
{
    let mut s = String::from("hello");
    println!("{s}, world!");

    s = String::from("ahoy");
    println!("{s}, world!");
}
```



<!--------------------------------------------------------------------------------- Function -->
<br><br>

## Function
Stack : Passing a variable to a function will copy
```rust
fn main()
{
    let x = 5;
    fn_1(x);

    println!("main {}", x);
}

fn fn_1(v: i32) 
{ 
    println!("fn_1 {}", v);
}
```

Heap : Passing a variable to a function will move
```rust
fn main()
{
    let s = String::from("hello");
    fn_1(s);

    //println!("main {}", s); //chon move shode dige s vojood nadarad
}

fn fn_1(v: String) 
{ 
    println!("fn_1 {}", v);
}
```

Return Values : Returning values can also transfer ownership
```rust
fn main() 
{
    let s = fn_1();
    println!("s: {}", s);
}

fn fn_1() -> String 
{
    let v = String::from("hello");
    v
}
```

Return Values : We want to let a function use a value but not take ownership
```rust
fn main() 
{
    let s = String::from("hello");
    let s = calculate_length(s);
    println!("s: {s}");
}

fn calculate_length(v: String) -> String
{
    let length = v.len();
    println!("length: {length}");
    v
}
```