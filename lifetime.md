# Lifetime
    Lifetimes ensure that references are valid as long as we need them to be
    The main aim of lifetimes is to prevent dangling references, which cause a program to reference data other than the data it’s intended to reference

    Dangling References :
        Eshare variable be jaee ke vojood nadarad
        1 - variable morde bashad
        2 - be jai ke nist eshare konad

    The Borrow Checker:
        The Rust compiler has a borrow checker that compares scopes to determine whether all borrows are valid


<!--------------------------------------------------------------------------------- Working -->
<br><br>

## Base

Alive | No-Refrence 
```rust
fn main() 
{
    let v: i32;
    println!("v: {}", v);
}
```
Alive | Refrenced 
```rust
fn main() 
{
    let v: i32 = 1;
    {
        println!("v1: {}", v);
    }
    println!("v2: {}", v);
}
```
Alive | Refrenced 
```rust
fn main() 
{
    let mut v: i32 = 1;
    {
        println!("v1: {}", v);
        v = 2; //Copy ownership has occurred.
        println!("v2: {}", v);
    }
    println!("v3: {}", v);
}
```
Alive | Refrenced 
```rust
fn main() 
{
    let mut v: i32 = 1;
    {
        println!("v1: {}", v);
        let x = 2;
        v = x; //Copy ownership has occurred.
        println!("v2: {}", v);
    }
    println!("v3: {}", v);
}
```
Alive | Refrenced 
```rust
fn main() 
{
    let str1 = String::from("aaa");
    let str2 = String::from("bbbbbbbbb");
    let result = fn_1(str1, str2); //Transfer ownership has occurred.
    println!("fn_1: {}", result);

    // println!("str1: {}", str1);
    // println!("str2: {}", str2);

    fn fn_1(s1: String, s2: String) -> String 
    {
        if s1.len() > s2.len() 
        {
            s1
        } 
        else 
        {
            s2
        }
        // vaghti ye variable return mikoni Transfer ownership etefagh miuftad
        // dar vaghe dari variable va ownershipesho Transfer mikoni birooon
    }
}
```
```rust
fn main() 
{
    let str1 = String::from("aaa");
    let str2 = String::from("bbbbbbbbb");
    let result = fn_2(&str1, &str2); // ownership transfer nashode
    println!("fn_1: {}", result);

    println!("str1: {}", str1);
    println!("str2: {}", str2);

    fn fn_2(s1: &str, s2: &str) -> String 
    {
        if s1.len() >= s2.len() 
        {
            s1.to_string()
        } 
        else 
        {
            s2.to_string()
        }
        // vaghti ye variable return mikoni Transfer ownership etefagh miuftad
        // dar vaghe dari variable va ownershipesho Transfer mikoni birooon
    }
}
```



<!--------------------------------------------------------------------------------- Dangling References  -->
<br><br>

## Dangling References 
```rust
fn main() 
{
    let v : &i32;
    {
        let x: i32 = 5;
        v = &x;
    }
    //x az beyn khahad raft va hamchenin refrence of x
    //pas v inja dar be refrence ke nist eshare mikone
    println!("v: {}", v);
}
```
```rust
fn main() 
{
    let v : &i32;
    change(v);

    //x az beyn khahad raft va hamchenin refrence of x
    //pas v inja dar be refrence ke nist eshare mikone
    println!("v: {}", v);

    fn change(i : & i32)
    {
        let x: i32 = 42;
        i = &x;
    }
}
```
```rust
fn main() 
{
    let str1 = String::from("aaa");
    let str2 = String::from("bbbbbbbbb");
    let result = fn_2(&str1, &str2);
    println!("fn_1: {}", result);

    fn fn_2(s1: &str, s2: &str) -> &str 
    {
        if s1.len() >= s2.len() 
        {
            s1
        } 
        else 
        {
            s2
        }
    }
    // akhare in function parametr ha yani s1 va s2 az beyn khahan raft
    // pas to dari chi ro return mikoni ?
    // bayad ye kari konim s1 va s2 zende bemonan
}
```



<!--------------------------------------------------------------------------------- Resolve -->
<br><br>

## Resolve
```rust
fn main() 
{
    let str1 = String::from("aaa");
    let str2 = String::from("bbbbbbbbb");
    let result = fn_2(&str1, &str2);
    println!("fn_1: {}", result);

    fn fn_2<'a>(s1: &'a str, s2: &'a str) -> &'a str 
    {
        if s1.len() >= s2.len() 
        {
            s1
        } 
        else 
        {
            s2
        }
    }
}
```



<!--------------------------------------------------------------------------------- Struct -->
<br><br>

## Struct
```rust
fn main() 
{
    struct Book<'a>
    {
        title: &'a str,
        author: &'a str,
    }

    impl Book
    {
        fn print()
        {
            println!("Book: {} by {}", book.title, book.author);
        }
    }


    let title = String::from("The Great Gatsby");
    let author = String::from("F. Scott Fitzgerald");
    let book = Book{title: &title, author: &author};
    println!("Book: {} by {}", book.title, book.author);
    println!("title: {}", title);
    println!("author: {}", author);
}
```
```rust
fn main() 
{
    struct Book<'a>
    {
        title: &'a str,
        author: &'a str,
    }

    impl Book<'_>
    {
        fn display<'a>(&self)
        {
            println!("Book: {} by {}", self.title, self.author);
        }
    }

    let title = String::from("The Great Gatsby");
    let author = String::from("F. Scott Fitzgerald");
    let book = Book{title: &title, author: &author};
    book.display();
}
```