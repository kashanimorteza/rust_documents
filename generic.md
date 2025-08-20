# Generic
    All example about Generic



<!--------------------------------------------------------------------------------- Function -->
<br><br>

## Function

```rust

```

```rust

```


<!--------------------------------------------------------------------------------- Struct -->
<br><br>

## Struct

```rust
fn main() 
{
    struct Point<T> 
    {
        x: T,
        y: T,
    }

    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    println!("integer: {} {}", integer.x, integer.y);
    println!("float: {} {}", float.x, float.y);
}
```
```rust
struct Point<T, U> 
{
    x: T,
    y: U,
}

fn main() 
{
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };

    println!("both_integer: {} {}", both_integer.x, both_integer.y);
    println!("both_float: {} {}", both_float.x, both_float.y);
    println!("integer_and_float: {} {}", integer_and_float.x, integer_and_float.y);
}
```
Method
```rust
fn main() 
{
    struct Point<T> 
    {
        x: T,
        y: T,
    }
    
    impl<T> Point<T> 
    {
        fn x(&self) -> &T 
        {
            &self.x
        }
    }

    let p = Point { x: 5, y: 10 };

    println!("p.x = {} {} ", p.x(), p.y);
}
```
```rust
fn main() 
{
    struct Point<T> 
    {
        x: T,
        y: T,
    }
    
    impl<T> Point<T> 
    {
        fn x(&self) -> &T {
            &self.x
        }
    }
    
    impl Point<f32> 
    {
        fn distance_from_origin(&self) -> f32 
        {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
    //println!("p.x = {}", p.distance_from_origin()); error

    let f = Point { x: 5.1, y: 10.1 };
    println!("f.x = {}", f.x());
    println!("f.x = {}", f.distance_from_origin());
}
```
```rust
fn main() 
{
    struct Point<X1, Y1> 
    {
        x: X1,
        y: Y1,
    }

    impl<X1, Y1> Point<X1, Y1> 
    {
        fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> 
        {
            Point 
            {
                x: self.x,
                y: other.y,
            }
        }
    }

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
```



<!--------------------------------------------------------------------------------- Enum -->
<br><br>

## Enum

Option
```rust
pub enum Option<T> 
{
    Some(T),
    None,
}
```

Result
```rust
enum Result<T, E> 
{
    Ok(T),
    Err(E),
}
```

```rust
fn main() 
{
    let a = MyEnum::Single(42);
    let b = MyEnum::Pair(1, 2);

    print_enum(a);
    print_enum(b);
}

enum MyEnum<T> 
{
    Single(T),
    Pair(T, T),
}

fn print_enum<T: std::fmt::Debug>(item: MyEnum<T>) 
{
    match item 
    {
        MyEnum::Single(val) => println!("Single: {:?}", val),
        MyEnum::Pair(x, y) => println!("Pair: {:?} and {:?}", x, y),
    }
}
```



