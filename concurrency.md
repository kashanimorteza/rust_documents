# Concurrency

    Process
    Threads : Spawn, Move
    Channel
    Shared-State
    Mutex
    Race conditions | Deadlocks

    Concurrency:
        When an individual works on several different tasks before any of them is complete

    Process:
        In most current operating systems, an executed program’s code is run in a process

    Threads :
        Within a program, you can also have independent parts that run simultaneously. The features that run these independent parts are called threads

    Race conditions:
        in which threads are accessing data or resources in an inconsistent order
    
    Deadlocks:
        in which two threads are waiting for each other, preventing both threads from continuing

    The calls to thread::sleep force a thread to stop its execution for a short duration, allowing a different thread to run



<!--------------------------------------------------------------------------------- Threads -->
<br><br>

## Threads
Simple
```rust
fn main() 
{
    use std::thread;

    thread::spawn(|| 
        {
            for i in 1..10 
            {
                println!("spawn: {i}");
            }
        }
    );

    for i in 1..6 
    {
        println!("main : {i}");
    }
}
```
```rust
fn main() 
{
    use std::thread;
    use std::time::Duration;

    thread::spawn(|| 
        {
            for i in 1..10 
            {
                println!("spawn: {i}");
            }
        }
    );

    for i in 1..6 
    {
        println!("main : {i}");
        thread::sleep(Duration::from_millis(1));
    }
}
```
```rust
fn main() 
{
    use std::thread;
    use std::time::Duration;

    thread::spawn(|| 
        {
            for i in 1..10 
            {
                println!("spawn: {i}");
                thread::sleep(Duration::from_millis(1));
            }
        }
    );

    for i in 1..5 
    {
        println!("main : {i}");
        thread::sleep(Duration::from_millis(1));
    }
}
```

Join
```rust
fn main() 
{
    use std::thread;

    let handle = thread::spawn(|| 
        {
            for i in 1..10 
            {
                println!("spawn: {i}");
            }
        }
    );

    for i in 1..5 
    {
        println!("main : {i}");
    }

    handle.join().unwrap();
}
```
```rust
fn main() 
{
    use std::thread;
    use std::time::Duration;

    let handle = thread::spawn(|| 
        {
            for i in 1..10
            {
                println!("spawn: {i}");
            }
        }
    );

    for i in 1..6
    {
        println!("main : {i}");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
```

Move
```rust
fn main() 
{
    use std::thread;
    
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| 
        {
            println!("Here's a vector: {v:?}");
        }
    );

    handle.join().unwrap();
}
```
```rust
fn main() 
{
    use std::thread;
    
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move|| 
        {
            println!("Here's a vector: {v:?}");
        }
    );

    handle.join().unwrap();
}
```
```rust
fn main() 
{
    use std::thread;
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || 
        {
            for num in v
            {
                println!("n: {}", num);
            }
            
        }
    );
    handle.join().unwrap();
    //println!("Here's a vector: {v:?}");
}
```
```rust
fn main() 
{
    use std::thread;
    let v = vec![1, 2, 3];
    let handle = thread::spawn(|| 
        {
            for num in v
            {
                println!("n: {}", num);
            }
            
        }
    );
    handle.join().unwrap();
    //println!("Here's a vector: {v:?}");
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
        In a way, channels in any programming language are similar to single ownership, because once you transfer a value down a channel, you should no longer use that value

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
    Using Mutexes to Allow Access to Data from One Thread at a Time
    mutex allows only one thread to access some data at any given time
    You must attempt to acquire the lock before using the data.
    When you’re done with the data that the mutex guards, you must unlock the data so other threads can acquire the lock.
    
    

The API of Mutex<T>
```rust
use std::sync::Mutex;

fn main() 
{
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {m:?}");
}
```

Sharing a Mutex<T> Between Multiple Threads
```rust
//Rust is telling us that we can’t move the ownership of lock counter into multiple threads

use std::sync::Mutex;
use std::thread;

fn main() 
{
    let counter = Mutex::new(0);
    let mut handles = vec![];

    for _ in 0..10 {
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles 
    {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

Multiple Ownership with Multiple Threads
```rust
use std::rc::Rc;
use std::sync::Mutex;
use std::thread;

fn main() 
{
    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Rc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles 
    {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

Atomic Reference Counting with Arc<T>
```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() 
{
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 
    {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles 
    {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```