# Smart pointer
    Box
    Deref
    Wrapper
    Dynamic box 



<!--------------------------------------------------------------------------------- Box -->
<br><br>

## Box
```rust
fn main() 
{
    let b:Box<i32> = Box::new(110);
    println!("Box: {}", b);
}
```
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



<!--------------------------------------------------------------------------------- Deref -->
<br><br>

## Deref
```rust
fn main() {
    let x: i32 = 5;
    let y: Box<i32> = Box::new(x);

    println!("x: {}", x);
    println!("*y: {}", *y);
}
```
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



<!--------------------------------------------------------------------------------- Wrapper -->
<br><br>

## Wrapper
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



<!--------------------------------------------------------------------------------- Dynamic box  -->
<br><br>

## Dynamic box 
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

