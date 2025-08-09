# Variable
    All example about variable
    



<!--------------------------------------------------------------------------------- Declar -->
<br><br>

## Declar



<!--------------------------------------------------------------------------------- Mutability -->
<br><br>

## Mutability
    by default, variables are immutable
    
Immutable
```rust
fn main() 
{
    let x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
```

Mutable
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

```rust
fn main()
{
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of x is: {spaces}");
}
```

Error
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



<!--------------------------------------------------------------------------------- Smart pointer -->
<br><br>

## Smart pointer

Box
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

Deref
```rust
fn main() {
    let x: i32 = 5;
    let y: Box<i32> = Box::new(x);

    println!("x: {}", x);
    println!("*y: {}", *y);
}
```

Deref
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

Deref
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

Wrapper
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

Dynamic box 
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



<!--------------------------------------------------------------------------------- Environment -->
<br><br>

## Environment

Install with cargo
```bash
cargo add dotenvy
```

Install with Cargo.toml
```rust
[dependencies]
dotenvy = "0.15"
```

Add variables
```bash
echo DATABASE_SQLITE_URL=sample.db > .env 
echo DATABASE_PGSQL_URL=postgres://raspberrypi_api:123456@127.0.0.1/raspberrypi_api >> .env
echo DATABASE_URL=sample.db >> .env 
echo PORT=8080 >> .env
```

Read variable
```rust
use std::env;
use dotenvy::dotenv;

fn main() 
{
    // Load .env file into std::env
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let api_key = env::var("API_KEY").unwrap_or_else(|_| "default_key".to_string());
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse()
        .expect("PORT must be a valid number");

    println!("DB: {}", db_url);
    println!("API KEY: {}", api_key);
    println!("PORT: {}", port);
}
```