# Variable
    All example about variable



<!--------------------------------------------------------------------------------- Declar -->
<br><br>

## Declar




<!--------------------------------------------------------------------------------- Mutability -->
<br><br>

## Mutability

Example 1 : Immutable
```rust
fn main() 
{
    let x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
```

Example 2 : Mutable
```rust
fn main() 
{
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
```



<!--------------------------------------------------------------------------------- Const -->
<br><br>

## Const

Example 1
```rust
fn main() 
{
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Const: {THREE_HOURS_IN_SECONDS}");
}
```



<!--------------------------------------------------------------------------------- Shadowing -->
<br><br>

## Shadowing

Example 1
```rust
fn main() 
{
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
}
```

Example 2
```rust
fn main()
{
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of x is: {spaces}");
}
```

Example 3 : error
```rust
fn main() 
{
    let mut spaces = "   ";
    spaces = spaces.len();
}
```



<!--------------------------------------------------------------------------------- Life Time -->
<br><br>

## Life Time

Example 1
```rust
fn main() {
    //----------1
    println!("\n1\n-----------");
    let str1 = String::from("hello");
    let str2 = String::from("world!!!");
    let result = fn_1(str1, str2);
    println!("fn_1: {}", result);
    
    //----------2
    println!("\n2\n-----------");
    let str3 = "hello";
    let str4 = "world!!!";
    let result = fn_2(str3, str4);
    println!("fn_2: {}", result);

    //----------3
    println!("\n3\n-----------");
    let str5 = "hello";
    let str6 = "world!!!";
    let result = fn_2(str3, str4);
    println!("fn_2: {}", result);

}


fn fn_1(s1: String, s2: String) -> String {
    if s1.len() >= s2.len() {
        s1
    } else {
        s2
    }
}

fn fn_2(s1: &str, s2: &str) -> String {
    if s1.len() >= s2.len() {
        s1.to_string()
    } else {
        s2.to_string()
    }
}
```

Example 2
```rust
fn main() {
    let string1: String = String::from("valueaaaaaasjbsajhdb");
    let string2: String = String::from("............");
    let result: &str = longest(&string1, &string2);
    println!("result: {}", result);
}

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}
```



<!--------------------------------------------------------------------------------- Pointer -->
<br><br>

## Pointer

Example 1
```rust
```

Example 2
```rust
```

Example 3
```rust
```



<!--------------------------------------------------------------------------------- Smart pointer -->
<br><br>

## Smart pointer

Example 1 : box
```rust
fn main() 
{
    let b:Box<i32> = Box::new(110);
    println!("Box: {}", b);
}
```

Example 2
```rust
struct Order
{
    id: u8,
    name: String,
    note:Box<String>,
}
fn main() 
{
    let note : String = String::from("note-1");
    let order: Order = Order
    {
        id:1,
        name: String::from("name-1"),
        note: Box::new(note),
    };
    println!("id: {}, name: {}, note: {} ", order.id, order.name, order.note);
}
```

Example 3
```rust
#[derive(Debug)]
enum List {
    Node(u32, Box<List>),
    Empty,
}
 
fn main() {
    let linked_list: List = List::Node(
        1,
        Box::new(List::Node(
            2,
            Box::new(List::Node( 
                3,
                Box::new(List::Empty),
            )),
        )),
    );
    println!("List {:?}", linked_list);
}
```

Example 4 : Deref
```rust
fn main() {
    let x: i32 = 5;
    let y: Box<i32> = Box::new(x);

    println!("x: {}", x);
    println!("*y: {}", *y);
}
```

Example 5 : Deref
```rust
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> 
{
    fn new(x: T) -> MyBox<T> 
    {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> 
{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() 
{
    let x: i32 = 5;
    let y: MyBox<i32> = MyBox::new(x);
    println!("x: {}", x);
    println!("*y: {}", *y);
}
```

Example 6 : Deref
```rust
fn hi(name: &str) 
{
    println!("Hi {}", name);
}

fn main() 
{
    let name: Box<String> = Box::new(String::from("Leen"));
    hi(&name);
}
```

Example 7 : Wrapper
```rust
use std::ops::Deref;

struct Config 
{
    verbose: bool,
}

struct SmartConfig(Box<Config>);

impl Deref for SmartConfig 
{
    type Target = Config;
    fn deref(&self) -> &Self::Target 
    {
        &self.0
    }
}

fn print_verbose(config: &Config) 
{
    println!("Verbose mode: {}", config.verbose);
}

fn main() {
    let config: SmartConfig = SmartConfig(
        Box::new(
            Config { verbose: false }
        )
    );
    print_verbose(&config);
}
```

Example 8 : Dynamic box 
```rust
trait Notifier  
{
    fn notify(&self);
}

struct Email;

struct SMS;

impl Notifier for Email 
{
    fn notify(&self) 
    {
        println!("Sending Email")
    }
}

impl Notifier for SMS 
{
    fn notify(&self) 
    {
        println!("Sending SMS")
    }
}

fn main() 
{
    let email = Box::new(Email);
    let sms = Box::new(SMS);
    let notification: Vec<Box<dyn Notifier>> = vec![email, sms];
    for n in notification
    {
        n.notify();
    }
}
```

Example 9
```rust
```

Example 10
```rust
```



<!--------------------------------------------------------------------------------- Declar -->
<br><br>

## Declar

Install with cargo  

    cargo add dotenvy

Install with Cargo.toml :

    [dependencies]
    dotenvy = "0.15"
