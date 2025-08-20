# Variable
    Declar
    Mutability
    Const
    Shadowing
    Environment

    Vector
    Refrence
    DeRefrence
    Life time
    Pointer
    Smart poiner



<!--------------------------------------------------------------------------------- Declar -->
<br><br>

## Declar

```rust
fn main() 
{
    let v_1:i8 = 1;
    let v_2 = 2;

    println!("The value of x is: {v_1}");
    println!("The value of x is: {v_2}");
}
```

```rust
fn main() 
{
    let (a, b, c) = (11, 12, 13);

    println!("a: {}", a);
    println!("b: {}", b);
    println!("c: {}", c);
}

```



<!--------------------------------------------------------------------------------- Mutability -->
<br><br>

## Mutability
    By default, variables are immutable

```rust
fn main() 
{
    let x = 5;
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}");
}
```

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
    Constants aren’t just immutable by default—they’re always immutable
    Constants can be declared in any scope, including the global scope
    Set only to a constant expression, not the result of a value that could only be computed at runtime
    The type of the value must be annotated
    Rust’s naming convention for constants is to use all uppercase with underscores between words

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

We can change value of variable
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

We can change the type of the value but reuse the same name
```rust
fn main()
{
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of x is: {spaces}");
}
```

```rust
//---Error 
fn main() 
{
    let mut spaces = "   ";
    spaces = spaces.len();
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