# Lifetime
    Lifetimes ensure that references are valid as long as we need them to be
    The main aim of lifetimes is to prevent dangling references, which cause a program to reference data other than the data it’s intended to reference

    The Borrow Checker:
        The Rust compiler has a borrow checker that compares scopes to determine whether all borrows are valid


<!--------------------------------------------------------------------------------- Working -->
<br><br>

## Working
```rust
fn main() 
{
    let v: i32;
    {
        v = 42;
        //Copy ownership has occurred.
    }
    println!("v: {}", v);
}
```
```rust
fn main() 
{
    let v: i32;
    {
        let x = 5;
        v = x;
        //Copy ownership has occurred.
        println!("x: {}", x);

    }
    println!("v: {}", v);
}
```
```rust
fn main() 
{
    let str1 = String::from("aaa");
    let str2 = String::from("bbbbbbbbb");
    let result = fn_1(str1, str2);
    println!("fn_1: {}", result);

    // Transfer ownership has occurred.
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
    let result = fn_2(&str1, &str2);
    println!("fn_1: {}", result);

    // ownership transfer nashode
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
```rust
fn main() 
{
    let str1 = String::from("aaa");
    let str2 = String::from("bbbbbbbbb");
    fn_2(&str1, &str2);

    // ownership transfer nashode
    println!("str1: {}", str1);
    println!("str2: {}", str2);
    

    fn fn_2(s1: &str, s2: &str)  
    {
        println!("s1: {} s2: {} ", s1, s2);
        //chizi nemikhay biroon bedi ke life time mani peyda konad
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
        //x az beyn khahad raft va hamchenin refrence of x
        // pas v dar be refrence ke nst eshare mikone
    }
    println!("v: {}", v);
}
```
```rust
fn main() 
{
    let z : & i32;
    change(z);
    println!("z: {}", z);

    fn change(v : & i32)
    {
        let x: i32 = 42;
        v = &x;
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