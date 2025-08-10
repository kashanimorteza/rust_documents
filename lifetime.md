# Lifetime
    All example about Lifetime


```rust
fn main() 
{
    let v;
    {
        v = 42;
    }
    println!("v: {}", v);
}
```

```rust
fn main() 
{
    let v;
    {
        let x = 5;
        v = &x;
    }
    println!("v: {}", v);
}
```

```rust
fn main() 
{
    let mut z : &mut i32;
    change(z);
    println!("z: {}", z);

    fn change(v : &mut i32)
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

```

```rust

```

```rust

```










