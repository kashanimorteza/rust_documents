# Concurrency
    All example about concurrency

    Process:
        In most current operating systems, an executed program’s code is run in a process

    Threads :
        Within a program, you can also have independent parts that run simultaneously. The features that run these independent parts are called threads

    Race conditions:
        in which threads are accessing data or resources in an inconsistent order
    
    Deadlocks:
        in which two threads are waiting for each other, preventing both threads from continuing

    The calls to thread::sleep force a thread to stop its execution for a short duration, allowing a different thread to run




<!--------------------------------------------------------------------------------- Structure -->
<br><br>

## Structure



<!--------------------------------------------------------------------------------- Install -->
<br><br>

## Install  



<!--------------------------------------------------------------------------------- Setup -->
<br><br>

## Setup
```rust
```

```rust
```

```rust
```

```rust
```


<!--------------------------------------------------------------------------------- Spawn -->
<br><br>

## Spawn

```rust
use std::thread;
use std::time::Duration;

fn main() 
{
    thread::spawn(|| {
        for i in 1..10 
        {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 
    {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
}
```

Waiting for All Threads to Finish Using join Handles
```rust
use std::thread;
use std::time::Duration;

fn main() 
{
    let handle = thread::spawn(|| {
        for i in 1..10 
        {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 
    {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
```

The thread currently running until the thread represented by the handle terminates
```rust
use std::thread;
use std::time::Duration;

fn main() 
{
    let handle = thread::spawn(|| {
        for i in 1..10 
        {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 
    {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
}
```

Using move Closures with Threads
```rust
use std::thread;

fn main() 
{
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Here's a vector: {v:?}");
    });

    handle.join().unwrap();
}
```

```rust
use std::thread;

fn main() 
{
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Here's a vector: {v:?}");
    });

    drop(v); // oh no!

    handle.join().unwrap();
}
```

```rust
use std::thread;

fn main() 
{
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });

    handle.join().unwrap();
}
```