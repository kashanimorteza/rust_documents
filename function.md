# Function
    Rust code uses snake case as the conventional style for function and variable names

    Function
    Closure
    Iterators
    Associated Function



<!--------------------------------------------------------------------------------- Base -->
<br><br>

## Base
Function
```rust
fn main() 
{ 
    fn_1();
}

fn fn_1() 
{
    println!("fn_1");
}
```
Parameters
```rust
fn main() 
{ 
    fn_2(10);
}

fn fn_2(x: i8) 
{
    println!("fn_2 {}", x);
}
```
```rust
fn main() 
{
    fn_1(5, 'h');
}

fn fn_1(p1: i32, p2: char) 
{
    println!("p1: {} p2:{}", p1, p2);
}
```
Return Values
```rust
fn main() 
{
    //---fn_1
    let var_fn_1: i8 = fn_1();
    println!("fn_1 {}", var_fn_1);
    //---fn_2
    let var_fn_2: i8 = fn_2();
    println!("fn_2 {}", var_fn_2);
}

fn fn_1() -> i8 
{
    return 16;
}

fn fn_2() -> i8 
{
    9
}
```
Parameters | Return Values
```rust
fn main() 
{
    let var_fn_5: i8 = fn_5(10);
    println!("fn_5 {}", var_fn_5);
}

fn fn_5(x: i8) -> i8 
{
    return x;
}
```



<!--------------------------------------------------------------------------------- Closure -->
<br><br>

## Closure

    Rust’s closures are anonymous functions you can save in a variable or pass as arguments to other functions

```rust
fn main() 
{
    let fn_1 = || println!("This is Closure without parameter");
    fn_1();
}
```
```rust
fn main() 
{
    let fn_1 = | v:u8 | println!("This is Closure with parameter {}", v);
    fn_1(1);
}
```
```rust
fn main() 
{
    let fn_1 = | v:u8 | 
    {
        println!("This is Closure with parameter {}", v);
        println!("This is Closure with parameter {}", v);
        println!("This is Closure with parameter {}", v);
    };
    fn_1(1);
}
```
```rust
fn main() 
{
    let fn_1 = | v:u8 | ->u8 {v+1};
    println!("fn_1: {}", fn_1(1));
}
```
```rust
fn main() 
{
    struct Product 
    {
        name: String,
        price: f64,
    }

    let min_price:f64 = 100.0;

    let products = vec![
        Product { name: String::from("p1"), price: 100.0 },
        Product { name: String::from("p2"), price: 150.0 },
        Product { name: String::from("p3"), price: 200.5 },
        Product { name: String::from("p4"), price: 250.0 },
    ];
    
    let new_products : Vec<&Product> = products.iter().filter(|p| p.price > min_price).collect();
    
    for product in new_products
    {
        println!("{} - {}$", product.name, product.price);
    }
}
```
Borrowing immutably
```rust
fn main() 
{
    let list = vec![1, 2, 3];
    
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || list.push(7);

    println!("Before calling closure: {list:?}");

    only_borrows();
    
    println!("After calling closure: {list:?}");
}
```
Borrowing mutably
```rust
fn main() 
{
    let mut list = vec![1, 2, 3];
    
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || list.push(7);

    //println!("Before calling closure: {list:?}");

    borrows_mutably();

    println!("After calling closure: {list:?}");
}
```
Taking ownership
```rust
use std::thread;
fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");
    thread::spawn(move || println!("From thread: {list:?}")).join().unwrap();
    //println!("After calling closure: {list:?}");
}
```
Moving Captured Values Out of Closures and the Fn Traits
```rust
impl<T> Option<T> 
{
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self 
        {
            Some(x) => x,
            None => f(),
        }
    }
}
```



<!--------------------------------------------------------------------------------- Iterator -->
<br><br>

## Iterator

next
```rust
fn main() 
{
    let numbers: Vec<i32> = vec!{1,2,3,4,5};
    let mut iter = numbers.iter();
    println!("number : {:?}", iter.next());
    println!("number : {:?}", iter.next());
    println!("number : {:?}", iter.next());
    println!("number : {:?}", iter.next());
    println!("number : {:?}", iter.next());
    println!("number : {:?}", iter.next());
}
```
filter
```rust
struct Product {
    name: String,
    price: f64,
}

fn main() 
{
    let min_price:f64 = 100.0;

    let products = vec![
        Product { name: String::from("p1"), price: 100.0 },
        Product { name: String::from("p2"), price: 150.0 },
        Product { name: String::from("p3"), price: 200.5 },
        Product { name: String::from("p4"), price: 250.0 },
    ];
    
    let new_products : Vec<&Product> = products.iter().
        filter(|p| p.price > min_price).
        collect();
    
    for product in new_products
    {
        println!("{} - {}$", product.name, product.price);
    }
}
```
map
```rust
fn main() {
    let prices = vec![100.0, 200.0, 42.0];

    let tax_rate = 1.09;

    let taxed_price: Vec<f64> = prices
        .iter()
        .map(|price| price * tax_rate)
        .collect();

    println!("Prices after tax: {:?}", taxed_price);
}
```
sum
```rust
fn main() {
    let payments = vec![100.0, 200.0, 42.0];
    let total:f64 = payments.iter().sum();
    println!("Total: {}$", total);
}
```
take
```rust
fn main() {
    let payments = vec![100.0, 200.0, 42.0, 500.0, 126.0, 800.8];
    let first_three: Vec<f64>= payments.iter().take(3).cloned().collect();
    println!("First three payments: {:?}", first_three);
}
```
filtre + map + sum
```rust
struct Order {
    amount: f64,
    status: String,
}

fn main() {
    let orders = vec![
        Order { amount: 1.0, status: String::from("Canceled") },
        Order { amount: 2.0, status: String::from("Canceled") },
        Order { amount: 3.0, status: String::from("Completed") },
    ];

    let total_completed: f64 = orders
        .iter()
        .filter(|order| order.status == "Completed")
        .map(|order| order.amount)
        .sum();

    println!("Total completed amount: {}", total_completed);
}
```
check time
```rust
use std::time::Instant;

fn main() {
    let numbers: Vec<u64> = (1..=500_000_000).collect();
    let start = Instant::now();
    let sum: u64 = numbers.iter().sum();
    let duration = start.elapsed();
    println!("Sum using iter(): {} Time taken: {:?}", sum, duration);
}
```
for
```rust
use std::time::Instant;

fn main() {
    let numbers: Vec<u64> = (1..=500_000_000).collect();
    let start = Instant::now();
    let mut sum: u64 = 0;
    for num in numbers
    {
        sum += num;
    }
    let duration = start.elapsed();
    println!("Sum using iter(): {} Time taken: {:?}", sum, duration);
}
```
par_iter
```rust
use std::time::Instant;
use rayon::prelude::*;

fn main() {
    let numbers: Vec<u64> = (1..=500_000_000).collect();
    let start = Instant::now();
    let sum: u64 = numbers.par_iter().sum();
    let duration = start.elapsed();
    println!("Sum using iter(): {} Time taken: {:?}", sum, duration);
}
```



<!--------------------------------------------------------------------------------- Associated Function -->
<br><br>

## Associated Function