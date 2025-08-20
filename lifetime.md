# Lifetime
    Lifetimes ensure that references are valid as long as we need them to be
    The main aim of lifetimes is to prevent dangling references, which cause a program to reference data other than the data it’s intended to reference

    The Borrow Checker:
        The Rust compiler has a borrow checker that compares scopes to determine whether all borrows are valid

    Dangling References : 
        Eshare variable be jaee ke vojood nadarad

<!--------------------------------------------------------------------------------- Working -->
<br><br>

## Base
Error | Because the reference doesn't point to anywhere.
```rust
fn main() 
{
    let v: i32;
    println!("v: {}", v);
}
```
Ok | Because the reference points to the value 1 at first
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
Ok | Because the reference points to the value 2 | Copy ownership | Value
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
Ok | Because the reference points to the value x | Copy ownership | Variable
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
Ok | Because the reference points to the value return | Transfer ownership | Return variable
```rust
fn main() 
{
    let str1 = String::from("aaa");
    let str2 = String::from("bbbbbbbbb");
    let result = fn_1(str1, str2); 
    println!("fn_1: {}", result);

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
        // Transfer ownership has occurred for return variable
    }
}
```
Ok | Because the reference points to the value return | Transfer ownership | Return variable
```rust
fn main() 
{
    let str1 = String::from("aaa");
    let str2 = String::from("bbbbbbbbb");
    let result = fn_2(&str1, &str2);
    println!("fn_1: {}", result);

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
        // Transfer ownership has occurred for return variable
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



<!--------------------------------------------------------------------------------- Function -->
<br><br>

## Function
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
```rust
fn main() 
{
    
    let str1 = String::from("aaa");
    let str2 = String::from("bbbbbbbbb");
    let result = fn_2(&str1, &str2);
    println!("fn_1: {}", result);

    fn fn_2<'a>(s1: &'a str, s2: &str) -> &'a str 
    {
        s1
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

    fn fn_2<'a>(s1: &str, s2: &str) -> &'a str 
    {
        let result = String::from("really long string");
        result.as_str()
    }
}
```
```rust
fn main() 
{
    let my_string = String::from("hello world");
    let word = first_word(&my_string[..]);
    println!("word: {}", word);

    fn first_word(s: &str) -> &str 
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
}
```
```rust
fn main() 
{
    let my_string_literal = "hello world";
    let word1 = first_word(&my_string_literal[..]);
    let word2 = first_word(my_string_literal);

    println!("word1: {}", word1);
    println!("word2: {}", word2);

    fn first_word(s: &str) -> &str 
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
}
```
```rust
use std::fmt::Display;
fn main() 
{
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2, "Today is someone's birthday!");
    println!("The longest string is {result}");

    fn longest<'a, T>(
        x: &'a str,
        y: &'a str,
        ann: T,
    ) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {ann}");
        if x.len() > y.len() { x } else { y }
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

<!--------------------------------------------------------------------------------- Static -->
<br><br>

## Static
```rust
fn main() 
{
    let s: &'static str = "I have a static lifetime.";
    println!("s: {}", s);
}   
```
```rust
fn main() 
{
    let str1: &'static str = "aaa";
    let str2: &'static str = "bbbbbbbbb";
    let result = fn_2(str1, str2);
    println!("fn_1: {}", result);

    fn fn_2(s1: &'static str, s2: &'static str) -> &'static str 
    {
        if s1.len() >= s2.len() { s1 } else { s2 }
    }
}

```