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



<!--------------------------------------------------------------------------------- Channel -->
<br><br>

## Channel
    Using Message Passing to Transfer Data Between Threads
    Do not communicate by sharing memory; instead, share memory by communicating

    Channel : 
        A channel is a general programming concept by which data is sent from one thread to another
        A channel has two halves: a transmitter and a receiver

```rust
use std::sync::mpsc;

fn main() 
{
    let (tx, rx) = mpsc::channel();
}
```

```rust
use std::sync::mpsc;
use std::thread;

fn main() 
{
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || 
    {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });
}
```

```rust
use std::sync::mpsc;
use std::thread;

fn main() 
{
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {received}");
}
```

Channels and Ownership Transference
```rust
use std::sync::mpsc;
use std::thread;

fn main() 
{
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        println!("val is {val}");
    });

    let received = rx.recv().unwrap();
    println!("Got: {received}");
}
```

Sending Multiple Values and Seeing the Receiver Waiting
```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() 
{
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals 
        {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx 
    {
        println!("Got: {received}");
    }
}
```

Creating Multiple Producers by Cloning the Transmitter
```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() 
{
    // --snip--

    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals 
        {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals 
        {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx 
    {
        println!("Got: {received}");
    }

    // --snip--
}
```



<!--------------------------------------------------------------------------------- Shared-State -->
<br><br>

## Shared-State
```rust

```

```rust

```

```rust

```
