# Condition
    If
    Match
    Let if



<!--------------------------------------------------------------------------------- If -->
<br><br>

## If 
```rust
fn main() 
{
    let number = 3;
    if number < 5 
    {
        println!("number < 5");
    } 
    else 
    {
        println!("number > 5");
    }
}
```

```rust
fn main() 
{
    let number = 6;

    if number % 4 == 0 
    {
        println!("number is divisible by 4");
    } 
    else if number % 3 == 0 
    {
        println!("number is divisible by 3");
    } 
    else if number % 2 == 0 
    {
        println!("number is divisible by 2");
    } 
    else 
    {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

```rust
fn main() 
{
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);
}
```



<!--------------------------------------------------------------------------------- Match -->
<br><br>

## Match
    With if, the condition needs to evaluate to a Boolean value, but here it can be any type

Simple
```rust
fn main() 
{
    let v = 2;
    
    match v
    {
        0 => println!("0"),
        1 => println!("1"),
        _ => println!("Not 0 and not 1"),
    }
}
```

Brackets
```rust
fn main() 
{
    let v = 2;
    
    match v
    {
        0 => println!("0"),
        1 => println!("1"),
        _ => 
        {   
            println!("Value not exit");
            println!("Value isnot 0 and 1");
        }
    }
}
```

Enum
```rust
fn main() 
{
    #[allow(dead_code)]
    enum Coin 
    {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    let coin = Coin::Dime;
    
    match coin 
    {
        Coin::Penny => println!("Penny"),
        Coin::Nickel => println!("Nickel"),
        Coin::Dime => println!("Dime"),
        Coin::Quarter => println!("Quarter"),
    }
}
```

Enum with Values
```rust
fn main() 
{
    #[allow(dead_code)]
    #[derive(Debug)]
    enum UsState 
    {
        Alabama,
        Alaska,
    }

    #[allow(dead_code)]
    enum Coin 
    {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    let coin = Coin::Quarter(UsState::Alabama);
    
    match coin 
    {
        Coin::Penny => println!("Penny"),
        Coin::Nickel => println!("Nickel"),
        Coin::Dime => println!("Dime"),
        Coin::Quarter(state) => println!("State quarter from {state:?}!"),
    }
}
```

Option<T>
```rust
fn main() 
{
    fn plus_one(x: Option<i32>) -> Option<i32> 
    {
        match x 
        {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
```

Exhaustive
```rust
fn main() 
{
    fn plus_one(x: Option<i32>) -> Option<i32> 
    {
        match x 
        {
            Some(i) => Some(i + 1),
            //None => None,
        }
    }

    let five = Some(5);
}
```

Catch-All
```rust
fn main() 
{
    let coin = 2;
    
    match coin 
    {
        0 => println!("0"),
        1 => println!("1"),
        other => println!("Not 0 and not 1 and value is {}", other),
    }
}
```

```rust
fn main() 
{
    let coin = 2;
    
    match coin 
    {
        0 => println!("0"),
        1 => println!("1"),
        _ => println!("Not 0 and not 1"),
    }
}
```

```rust
fn main() 
{
    let coin = 2;
    
    match coin 
    {
        0 => println!("0"),
        1 => println!("1"),
        _ => {},
    }
}
```

```rust
fn main() 
{
    let coin = 2;
    
    match coin 
    {
        0 => println!("0"),
        1 => println!("1"),
        _ => (),
    }
}
```


<!--------------------------------------------------------------------------------- If let -->
<br><br>

## If let

```rust
fn main() 
{
    let mut count =0; 
    let  config_max = Some(10);
    //let config_max: Option<i32> = None;
    match config_max 
    {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => count +=1,
    }
    println!("count : {}", count)
}
```

```rust
fn main() 
{
    let mut count =0; 
    //let  config_max = Some(10);
    let config_max: Option<i32> = None;
    if let Some(max) = config_max 
    {
        println!("The maximum is configured to be {max}");
    }
    else
    {
        count +=1
    }
    println!("count : {}", count)
}
```

```rust
fn main() 
{
    #[allow(dead_code)]
    enum Coin 
    {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    #[allow(dead_code)]
    #[derive(Debug)]
    enum UsState 
    {
        Alabama,
        Alaska,
    }

    impl UsState 
    {
        fn existed_in(&self, year: u16) -> bool 
        {
            match self 
            {
                UsState::Alabama => year >= 1819,
                UsState::Alaska => year >= 1959,
            }
        }
    }


    fn describe_state_quarter(coin: Coin) -> Option<String> 
    {
        if let Coin::Quarter(state) = coin 
        {
            if state.existed_in(1900) 
            {
                Some(format!("{state:?} is pretty old, for America!"))
            } 
            else 
            {
                Some(format!("{state:?} is relatively new."))
            }
        } 
        else 
        {
            None
        }
    }

    if let Some(desc) = describe_state_quarter(Coin::Quarter(UsState::Alaska)) 
    {
        println!("{desc}");
    }
}
```