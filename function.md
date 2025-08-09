# Function
    Function | Parameters | Return
    Associated Function
    Generic
    Traits
    Closure
    Iterators



<!--------------------------------------------------------------------------------- Function -->
<br><br>

## Function
Simple
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

Input
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

Output
```rust
fn main() 
{
    //---fn_3
    let var_fn_3: i8 = fn_3();
    println!("fn_3 {}", var_fn_3);
    //---fn_4
    let var_fn_4: i8 = fn_4();
    println!("fn_4 {}", var_fn_4);
}

fn fn_3() -> i8 
{
    return 16;
}

fn fn_4() -> i8 
{
    9
}
```

Input | Output
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



<!--------------------------------------------------------------------------------- Associated Function -->
<br><br>

## Associated Function



<!--------------------------------------------------------------------------------- Generic -->
<br><br>

## Generic



<!--------------------------------------------------------------------------------- Traits -->
<br><br>

## Traits



<!--------------------------------------------------------------------------------- Closure -->
<br><br>

## Closure

```rust
fn main() 
{
    let fn_1 = |v: u8| v + 1;

    let fn_1_result = fn_1(10);
    println!("fn_1: {}", fn_1_result);

    let fn_2_result = fn_2(10);
    println!("fn_2: {}", fn_2_result);
}

fn fn_2(v: u8) -> u8
{
    return v+1;
}
```

```rust
fn main() 
{
    let discount:f64 = 0.1;
    let apply_discount = |v: f64 | v - discount;
    println!("apply_discount : {}$", apply_discount(50.0));
}
```

```rust
fn main() 
{
    let mut discount:f64 = 0.1;
    {
        discount +=0.1;
    }
    let apply_discount = |v: f64 | v - discount;
    println!("apply_discount : {}$", apply_discount(50.0));
}
```

```rust
fn main() 
{
    let discount:f64 = 0.1;
    let apply_discount = |v: f64 | {
        println!("Price : {}$", v);
        println!("apply_discount : {}$", v - discount);
    };
    apply_discount(50.0);
}
```

```rust
fn main() 
{
    let mut visitor_count:u8 = 0;
    let mut add_visitor = || {
        visitor_count +=1;
        println!("new visitor... total : {}", visitor_count);
    };
    add_visitor();
    add_visitor();
    add_visitor();
    add_visitor();
}
```

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
    
    let new_products : Vec<&Product> = products.iter().filter(|p| p.price > min_price).collect();
    
    for product in new_products
    {
        println!("{} - {}$", product.name, product.price);
    }
}
```



<!--------------------------------------------------------------------------------- Iterators -->
<br><br>

## Iterators

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