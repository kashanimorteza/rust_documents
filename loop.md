# Loop
    All example about Loop


<!--------------------------------------------------------------------------------- for -->
<br><br>

## Loop

Example 1 : Simple
```rust
fn main() 
{
    let mut counter:i32 = 0;
    loop 
    {
        if counter>100
        {
            break;
        }
        print!("Loop {} \n", counter);
        counter +=1;
    }
}
```

Example 2 : Lable
```rust
fn main() {
    let mut count = 0;
    'counting_up: loop 
    {
        println!("count = {count}");
        let mut remaining = 10;
        loop 
        {
            println!("remaining = {remaining}");
            if remaining == 9 
            {
                break;
            }
            if count == 2 
            {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}
```

<!--------------------------------------------------------------------------------- for -->
<br><br>

## For

Example 1
```rust
fn main() 
{
    let counter= 0..100;
    for  c in counter 
    {
        print!("for {} \n", c);
    }
}
```



<!--------------------------------------------------------------------------------- While -->
<br><br>

## While

Example 1
```rust
fn main() 
{
    let mut counter:i32 = 0;
    while counter <100 
    {
        print!("While {} \n", counter);
        counter +=1;    
    }
}
```