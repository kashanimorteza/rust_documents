<!--------------------------------------------------------------------------------- Method -->
# Error
    All example about error
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

## abort & unwind

abort

    Ends the program without cleaning up

    [profile.release]
    panic = 'abort'



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

```rust

```

```rust

```

```rust

```



<!--------------------------------------------------------------------------------- Multiple error types -->
<br><br>

## Multiple error types




<!--------------------------------------------------------------------------------- Iterating over Results -->
<br><br>

## Iterating over Results