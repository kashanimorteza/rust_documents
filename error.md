<!--------------------------------------------------------------------------------- Error Handling -->
# Error Handling
    panic
    abort
    unwrap
    expect
    Result
    Multiple error types
    Iterating over Results

    unwrap()

    Recoverable :
        such as a file not found error, we most likely just want to report the problem to the user and retry the operation
        Result

    Unrecoverable:
        such as trying to access a location beyond the end of an array, and so we want to immediately stop the program
        panic

    Debug symbols are enabled by default when using cargo build or cargo run without the --release flag

    RUST_BACKTRACE=full cargo run



<!--------------------------------------------------------------------------------- panic -->
<br><br>

## panic
```rust
fn main() 
{
    panic!("crash and burn");
}
```

```rust
fn main() 
{
    let v = vec![1, 2, 3];
    v[99];
}
```



<!--------------------------------------------------------------------------------- abort & unwind -->
<br><br>

## abort

abort

    Ends the program without cleaning up

    [profile.release]
    panic = 'abort'



<!--------------------------------------------------------------------------------- unwrap -->
<br><br>

## unwrap
    If the Result value is the Ok variant, unwrap will return the value inside the Ok. If the Result is the Err variant, unwrap will call the panic! macro for us
```rust
use std::fs::File;

fn main() 
{
    let _greeting_file = File::open("hello.txt").unwrap();
    println!("-----------result------ {:?}", _greeting_file);
}
```



<!--------------------------------------------------------------------------------- expect -->
<br><br>

## expect
    Lets us also choose the panic! error message
```rust
use std::fs::File;

fn main() 
{
    let _greeting_file = File::open("hello.txt").expect("Some problem");
    println!("-----------result------ {:?}", _greeting_file);
}
```



<!--------------------------------------------------------------------------------- Result -->
<br><br>

## Result

```rust
enum Result<T, E> 
{
    Ok(T),
    Err(E),
}
```

```rust
use std::fs::File;

fn main() 
{
    let _greeting_file_result = File::open("hello.txt");
    print!("{:?}", _greeting_file_result);
}
```

```rust
use std::fs::File;

fn main() 
{
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result 
    {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };
    print!("greeting_file : {:?}", greeting_file);
}
```

Matching on Different Errors
```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() 
{
    let greeting_file_result = File::open("hello.txt");

    println!("-----------result------ {:?}", greeting_file_result);
    
    let _greeting_file = match greeting_file_result 
    {
        Ok(file) => file,
        Err(error) => match error.kind() 
        {
            ErrorKind::NotFound => match File::create("hello.txt") 
            {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => 
            {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };
}
```

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() 
{
    let _greeting_file = File::open("hello.txt").unwrap_or_else(|error| 
    {
        if error.kind() == ErrorKind::NotFound 
        {
            File::create("hello.txt").unwrap_or_else(|error| 
            {
                panic!("Problem creating the file: {error:?}");
            })
        } 
        else 
        {
            panic!("Problem opening the file: {error:?}");
        }
    });
}
```



<!--------------------------------------------------------------------------------- Propagating Errors -->
<br><br>

## Propagating Errors
    When a function’s implementation calls something that might fail, instead of handling the error within the function itself you can return the error to the calling code so that it can decide what to do
```rust
use std::fs::File;
use std::io::{self, Read};

fn main() 
{
    let res = read_username_from_file();
    let mut _res2 = match res 
    {
        Ok(_v) => println!("Ok"),
        Err(_e) => println!("error"),
    };
}

fn read_username_from_file() -> Result<String, io::Error> 
{
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result 
    {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) 
    {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
```

? : use the ? operator in a function that returns Result, Option, or another type that implements FromResidual
```rust
use std::fs::File;
use std::io::{self, Read};

fn main() 
{
    let res = read_username_from_file();
    let mut _res2 = match res 
    {
        Ok(_v) => println!("Ok"),
        Err(_e) => println!("error"),
    };
}

fn read_username_from_file() -> Result<String, io::Error> 
{
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
```

```rust
#![allow(unused)]
fn main() 
{
    use std::fs::File;
    use std::io::{self, Read};

    fn read_username_from_file() -> Result<String, io::Error> 
    {
        let mut username = String::new();
        File::open("hello.txt")?.read_to_string(&mut username)?;
        Ok(username)
    }
}
```

```rust
use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> 
{
    fs::read_to_string("hello.txt")
}
```