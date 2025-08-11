# Type
    Every value in Rust is of a certain data type
    The compiler can usually infer what type we want to use based on the value and how we use it.

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
    Collection
    Pointer and Reference
    Miscellaneous



<!--------------------------------------------------------------------------------- Integer -->
<br><br>

## Integer
Signed

    i8    1     Bit
    i16   2     Bits
    i32   4     Bits
    i64   8     Bits
    i128  16    Bits
    isize 32/64 Bits -> arch dependent

```rust
fn main() 
{
    let a: i8   = -128;
    let b: i16  = -32_768;
    let c: i32  = -2_147_483_648;
    let d: i64  = -9_223_372_036_854_775_808;
    let e: i128 = -170_141_183_460_469_231_731_687_303_715_884_105_728;
    let f: isize = -2147483648; // depends on your CPU arch (32-bit or 64-bit)

    println!("i8   : {}", a);
    println!("i16  : {}", b);
    println!("i32  : {}", c);
    println!("i64  : {}", d);
    println!("i128 : {}", e);
    println!("isize: {}", f);
}
```

Unsigned

    u8    1     Bit
    u16   2     Bits
    u32   4     Bits
    u64   8     Bits
    u128  16    Bits
    usize 32/64 Bits -> arch dependent

```rust
fn main() 
{
    let g: u8   = 255;
    let h: u16  = 65_535;
    let i: u32  = 4_294_967_295;
    let j: u64  = 18_446_744_073_709_551_615;
    let k: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455;
    let l: usize = 4_294_967_295; // depends on CPU arch

    println!("u8   : {}", g);
    println!("u16  : {}", h);
    println!("u32  : {}", i);
    println!("u64  : {}", j);
    println!("u128 : {}", k);
    println!("usize: {}", l);
}
```



<!--------------------------------------------------------------------------------- Float -->
<br><br>

## Float
    f32   32 Bits
    f64   64 Bits

```rust
fn main() 
{
    let is_active: bool = true;
    let is_complete: bool = false;

    println!("is_active: {}", is_active);
    println!("is_complete: {}", is_complete);
}
```



<!--------------------------------------------------------------------------------- Boolean -->
<br><br>

## Boolean
    bool        1 Bits

```rust
fn main() 
{
    let is_active: bool = true;
    let is_complete: bool = false;

    println!("is_active: {}", is_active);
    println!("is_complete: {}", is_complete);
}
```



<!--------------------------------------------------------------------------------- Char -->
<br><br>

## Char
    char        4 Bits

```rust
fn main() 
{
    let letter: char = 'A';
    let digit: char = '7';
    let symbol: char = '#';
    let emoji: char = '😊';

    println!("letter: {}", letter);
    println!("digit: {}", digit);
    println!("symbol: {}", symbol);
    println!("emoji: {}", emoji);
}
```



<!--------------------------------------------------------------------------------- Tuple -->
<br><br>

## Tuple
    Store on stack
    The way to have a collection of multiple values is with an array
    Tuples have a fixed length: once declared, they cannot grow or shrink in size
    Each position in the tuple has a type, and the types of the different values in the tuple don’t have to be the same

```rust
fn main() 
{
    let person: (&str, i32, bool) = ("Alice", 30, true);
    println!("Tuple: {:?}", person);

    println!("Name: {}", person.0);
    println!("Age: {}", person.1);
    println!("Is Active: {}", person.2);
}
```

```rust
fn main() 
{
    let person: (&str, i32, bool) = ("Alice", 30, true);
    let (name, age, active) = person;
    println!("{} is {} years old. Active? {}", name, age, active);
}
```



<!--------------------------------------------------------------------------------- Array -->
<br><br>

## Array
    Store on stack
    The way to have a collection of multiple values is with an array
    Every element of an array must have the same type
    Arrays in Rust have a fixed length
    Arrays are useful when you want your data allocated on the stack
    Arrays are more useful when you know the number of elements will not need to change

```rust
fn main() 
{
    let a = [1, 2, 3];
    println!("Array: {:?}", a);
    println!("Item: {}", a[0]);
}
```

```rust
#![allow(unused)]
fn main() 
{
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Item: {:?}", a);
}
```

```rust
#![allow(unused)]
fn main() 
{
    let a = [3; 5];
    println!("Item: {:?}", a);
    println!("Item: {}", a[0]);
}
```

```rust
#![allow(unused)]
fn main() 
{
    let a = [
        "January", "February", "March", "April", "May", "June", "July","August", "September", "October", "November", "December"
    ];
    println!("Item: {:?}", a);
}
```



<!--------------------------------------------------------------------------------- Structs -->
<br><br>

## Structs
    Structure, is a custom data type that lets you package together and name multiple related values that make up a meaningful group

Simple
```rust
fn main() 
{
    let user1 = User 
    {
        active: true,
        username: String::from("my_username"),
        email: String::from("my_email"),
        sign_in_count: 1,
    };

    println!("{:?}", user1);
    println!("{}", user1.active);
    println!("{}", user1.username);
    println!("{}", user1.email);
    println!("{}", user1.sign_in_count);
}

#[derive(Debug)]
struct User 
{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```
Simple : Change item
```rust
fn main() 
{
    let mut user1 = User 
    {
        active: true,
        username: String::from("my_username"),
        email: String::from("my_email"),
        sign_in_count: 1,
    };
    
    user1.email = String::from("my_email_2");
    println!("{}", user1.email);
}

#[derive(Debug)]
struct User 
{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```
Simple : Error
```rust
fn main() 
{
    let user1 = User 
    {
        active: true,
        username: String::from("my_username"),
        email: String::from("my_email"),
        sign_in_count: 1,
    };

    println!("{:?}", user1);
    println!("{}", user1.active);
    println!("{}", user1.username);
    println!("{}", user1.email);
    println!("{}", user1.sign_in_count);
}

#[derive(Debug)]
struct User 
{
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}
```
Creating Instances from Other Instances
```rust
fn main() 
{
    let user1 = User 
    {
        active: true,
        username: String::from("my_username_1"),
        email: String::from("my_email_1"),
        sign_in_count: 1,
    };

    let user2 = User 
    {
        active: user1.active,
        username: String::from("my_username_2"),
        email: String::from("my_email_2"),
        sign_in_count: user1.sign_in_count,
    };
    
    println!("user1 : {:?}", user1);
    println!("user2 : {:?}", user2);
}

#[derive(Debug)]
struct User 
{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```
```rust
fn main() 
{
    let user1 = User 
    {
        active: true,
        username: String::from("my_username"),
        email: String::from("my_email_1"),
        sign_in_count: 1,
    };

    let user2 = User 
    {
        active: user1.active,
        username: user1.username,
        email: String::from("my_email_2"),
        sign_in_count: user1.sign_in_count,
    };
    
    //user1.username not eable bucuse moved
    //println!("user1 : {:?}", user1);
    
    println!("user1 email : {:?}", user1.email);
    println!("user2 : {:?}", user2);
}

#[derive(Debug)]
struct User 
{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```
```rust
fn main() 
{
    let user1 = User 
    {
        active: true,
        username: String::from("my_username"),
        email: String::from("my_email_1"),
        sign_in_count: 1,
    };

    let user2 = User 
    {
        active: false,
        ..user1
    };
    
    //username and email moved and not eable bucuse
    //println!("user1 : {:?}", user1);

    println!("user1 active : {:?}", user1.active);
    println!("user2 : {:?}", user2);
}

#[derive(Debug)]
struct User 
{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```
Creating Instances with function
```rust
fn main() 
{
    let user1 = build_user
    (
        String::from("my_email@example.com"),
        String::from("my_username"),
    );
    println!("{:#?}", user1);
}

fn build_user(email: String, username: String) -> User 
{
    User 
    {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

#[derive(Debug)]
struct User 
{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```
```rust
fn main() {
    let user1 = build_user
    (
        String::from("my_email@example.com"),
        String::from("my_username"),
    );
    println!("{:#?}", user1);
}

fn build_user(email: String, username: String) -> User 
{
    User 
    {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

#[derive(Debug)]
struct User 
{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```
Tuple
```rust
struct Color(i32, i32, i32);

fn main() 
{
    let color = Color(0, 0, 0);
    println!("{} {} {}", color.0, color.1, color.2);
}
```
Unit-Like
```rust
struct AlwaysEqual;

fn main() 
{
    let subject = AlwaysEqual;
}
```
Method : Associated Functions
```rust
fn main() 
{
    let book = Book::new("t", "a", 120);
    println!("{} {} {}", book.title, book.author, book.pages);
}

#[derive(Debug)]
struct Book
{
    title: String,
    author: String,
    pages: u32,
} 

impl Book 
{
    fn new(title: &str, author: &str, pages: u32) -> Self
    {
        Self
        {
            title: String::from(title),
            author: String::from(author),
            pages: pages,
        }
    }
}
```
Method : with Parameters
```rust
#[derive(Debug)]
struct Rectangle 
{
    width: u32,
    height: u32,
}

impl Rectangle 
{
    fn area(&self) -> u32 
    {
        self.width * self.height
    }
}

fn main() 
{
    let rect1 = Rectangle 
    {
        width: 30,
        height: 50,
    };

    println!("The area of the rectangle is {} square pixels.", rect1.area());
}
```
Method : More Parameters
```rust
#[derive(Debug)]
struct Rectangle 
{
    width: u32,
    height: u32,
}

impl Rectangle 
{
    fn can_hold(&self, other: &Rectangle) -> bool 
    {
        self.width > other.width && self.height > other.height
    }
}

fn main() 
{
    let rect1 = Rectangle 
    {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle 
    {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle 
    {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
```
Method : Note that we can choose to give a method the same name as one of the struct’s fields
```rust
#[derive(Debug)]
struct Rectangle 
{
    width: u32,
    height: u32,
}

impl Rectangle 
{
    fn width(&self) -> bool 
    {
        self.width > 0
    }
}

fn main() 
{
    let rect1 = Rectangle 
    {
        width: 30,
        height: 50,
    };

    if rect1.width() 
    {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}
```
Method : Multiple impl Blocks
```rust
#[derive(Debug)]
struct Rectangle 
{
    width: u32,
    height: u32,
}

impl Rectangle 
{
    fn area(&self) -> u32 
    {
        self.width * self.height
    }
}

impl Rectangle 
{
    fn can_hold(&self, other: &Rectangle) -> bool 
    {
        self.width > other.width && self.height > other.height
    }
}

fn main() 
{
    let rect1 = Rectangle 
    {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle 
    {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle 
    {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
```
App : Simple
```rust
fn main() 
{
    let width1 = 30;
    let height1 = 50;
    println!("The area of the rectangle is {} square pixels.", area(width1, height1));
}

fn area(width: u32, height: u32) -> u32 
{
    width * height
}
```
App : Refactoring with Tuples
```rust
fn main() 
{
    let rect1 = (30, 50);
    println!("The area of the rectangle is {} square pixels.", area(rect1));
}

fn area(dimensions: (u32, u32)) -> u32 
{
    dimensions.0 * dimensions.1
}
```
// App : Refactoring with Structs
```rust
struct Rectangle 
{
    width: u32,
    height: u32,
}

fn main() 
{
    let rect1 = Rectangle 
    {
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is {} square pixels.", area(&rect1));
}

fn area(rectangle: &Rectangle) -> u32 
{
    rectangle.width * rectangle.height
}
```



<!--------------------------------------------------------------------------------- Enums -->
<br><br>

## Enums
```rust
fn main() 
{
    #[allow(dead_code)]
    #[derive(Debug)]
    enum IpAddrKind 
    {
        V4,
        V6,
    }

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("four: {:?}", four);
    println!("six: {:?}", six);
}
```
```rust
fn main() 
{
    enum IpAddr 
    {
        V4(String),
        V6(String),
    }

    let v4 = IpAddr::V4(String::from("127.0.0.1"));
    let v6 = IpAddr::V6(String::from("::1"));

    if let IpAddr::V4(val) = v4 
    {
        println!("The value of v4 is: {}", val);
    }

    if let IpAddr::V6(val) = v6
    {
        println!("The value of v6 is: {}", val);
    }
    
}
```
```rust
fn main() 
{
    enum IpAddr 
    {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
}
```
```rust
#![allow(unused)]
fn main() 
{
    struct Ipv4Addr 
    {
        // --snip--
    }

    struct Ipv6Addr 
    {
        // --snip--
    }

    enum IpAddr 
    {
        V4(Ipv4Addr),
        V6(Ipv6Addr),
    }
}
```
```rust
fn main() 
{
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}

enum IpAddrKind 
{
    V4,
    V6,
}

fn route(ip_kind: IpAddrKind) 
{

}
```
```rust
fn main() 
{
    enum IpAddrKind 
    {
        V4,
        V6,
    }

    struct IpAddr 
    {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr 
    {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr 
    {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}
```
```rust
fn main() 
{
    enum Message 
    {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message 
    {
        fn call(&self) 
        {
            // method body would be defined here
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
}
```
```rust
fn main() 
{
    // use std::option::Option::{Some, None};
    
    let some_number: Option<i32> = Some(42);
    let no_number: Option<i32> = None;

    match some_number 
    {
        Some(n) => println!("We have: {}", n),
        None => println!("No value"),
    }
}
```



<!--------------------------------------------------------------------------------- String -->
<br><br>

## String 

&str
```rust
fn main() 
{
    let s ="hello";
    println!("s: {}", s);
}
```

String
```rust
fn main() 
{
    let s1 = String::new();
    println!("s1: {}", s1);

    let data = "initial contents";
    let s2 = data.to_string();
    println!("s2: {}", s2);

    let s3 = "initial contents".to_string();
    println!("s3: {}", s3);

    let s4 = String::from("initial contents");
    println!("s4: {}", s4);
}
```

Updating a String
```rust
fn main() 
{
    let mut s = String::from("foo");
    s.push_str("bar");

    println!("s: {}", s);
}
```

```rust
fn main() 
{
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);
}
```

Concatenation with the + Operator or the format! Macro
```rust
fn main() 
{
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("s3 is {}", s3); // note s1 has been moved here and can no longer be used
}
```

```rust
fn main() 
{
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");
    println!("s is {}", s);
}
```

Slicing Strings
```rust
#![allow(unused)]
fn main() 
{
    let hello: &'static str = "hello";
    let s = &hello[0..1];
    println!("s is {}", s);
}
```

Methods for Iterating Over Strings
```rust
#![allow(unused)]
fn main() 
{
    for c in "hello".chars() 
    {
        println!("char is {}", c);
    }
}
```

```rust
#![allow(unused)]
fn main() 
{
    for b in "hello".bytes() 
    {
        println!("b is {}", b);
    }
}
```


<!--------------------------------------------------------------------------------- Vectors -->
<br><br>

## Vectors
    Vectors can only store values of the same type

Create
```rust
fn main() 
{
    let mut v = Vec::new();
    println!("v: {:?}", v);
}
```

Updating
```rust
fn main() 
{
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    println!("v: {:?}", v);
}
```

Reading Elements
```rust
fn main() 
{
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {third}");
}
```
```rust
fn main() 
{
    let v = vec![1, 2, 3, 4, 5];
    let third: Option<&i32> = v.get(2);
    match third 
    {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
}
```

Compiling this code will result in this error
```rust
fn main() 
{
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    v.push(6);
    println!("The first element is: {}", first);
}
```

Iterating
```rust
fn main() 
{
    let v = vec![100, 32, 57];
    for i in &v 
    {
        println!("{i}");
    }
}
```
```rust
fn main() 
{
    let mut v = vec![100, 32, 57];
    for i in &mut v 
    {
        *i += 50;
        println!("{i}");   
    }
}
```
```rust
fn main() 
{
    enum SpreadsheetCell 
    {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let v = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for cell in &v 
    {
        match cell 
        {
            SpreadsheetCell::Int(i) => println!("Int: {}", i),
            SpreadsheetCell::Float(f) => println!("Float: {}", f),
            SpreadsheetCell::Text(s) => println!("Text: {}", s),
        }
    }
}

```




<!--------------------------------------------------------------------------------- Slice -->
<br><br>

## Slice
    A string slice is a reference to part of a String, and it looks like this:

subject
```rust
fn main() 
{
    let s = String::from("salasm ali chetori");
    let l = first_word(&s);
    println!("Lenght: {l}");
}

fn first_word(s: &String) -> usize 
{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() 
    {
        if item == b' ' 
        {
            return i;
        }
    }
    s.len()
}
```
problem
```rust
fn main() 
{
    let mut s = String::from("salasm ali chetori");
    let l = first_word(&s);
    println!("Lenght: {l}");
    
    // afther that whats happen for Lenght???
    s.clear();
}

fn first_word(s: &String) -> usize 
{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() 
    {
        if item == b' ' 
        {
            return i;
        }
    }
    s.len()
}
```
String
```rust
fn main() 
{
    let s = String::from("hello world");
    let s1 = &s[0..5];
    let s2 = &s[6..11];
    println!("s1: {s1}");
    println!("s2: {s2}");
}
```
```rust
fn main() 
{
    let s = String::from("hello world");
    let s1 = &s[0..5];
    let s2 = &s[..5];
    println!("s1: {s1}");
    println!("s2: {s2}");
}
```
```rust
fn main() 
{
    let s = String::from("hello world");
    let s1 = &s[6..11];
    let s2 = &s[6..];
    println!("s1: {s1}");
    println!("s2: {s2}");
}
```
```rust
fn main() 
{
    let s = String::from("hello world");
    let s1 = &s[0..11];
    let s2 = &s[..];
    println!("s1: {s1}");
    println!("s2: {s2}");
}
```
```rust
fn main() 
{
    let s = String::from("salasm ali chetori");
    let f = first_word(&s);
    println!("first_word: {f}");
}

fn first_word(s: &String) -> &str 
{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() 
    {
        if item == b' ' 
        {
            return &s[0..i];
        }
    }
    &s[..]
}
```
```rust
#![allow(unused)]
fn main() 
{
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}
```


<!--------------------------------------------------------------------------------- Collection -->
<br><br>

## Collection

### Vec<T>
    Growable array
### VecDeque<T>
    Double-ended queue
### HashMap<K, V>
    Key-value hash map
### BTreeMap<K, V>
    Sorted map
### HashSet<T>
    Set without duplicates
### BTreeSet<T>
    Sorted set
### LinkedList<T>
    Doubly linked list
### BinaryHeap<T>
    Priority queue (max heap)



<!--------------------------------------------------------------------------------- Pointer and Reference -->
<br><br>

## Pointer and Reference

### &T	
    Shared reference
### &mut T	
    Mutable reference
### Box<T>	
    Heap-allocated smart pointer
### Rc<T>	
    Reference-counted pointer
### Arc<T>	
    Atomic reference counting
### *const T	
    Raw const pointer
### *mut T
    Raw mutable pointer



<!--------------------------------------------------------------------------------- Miscellaneou -->
<br><br>

## Miscellaneou

### Option<T>
    Nullable type (`Some`, `None`)
### Result<T, E>
    Success/error handling
### !
    Never type (doesn’t return)
### ()
    Unit type (like `void`)
### PhantomData<T>
    Marker for zero-sized types
