# OOP
    Objects
    Encapsulation
    Inheritance
    Polymorphism



<!--------------------------------------------------------------------------------- Object -->
<br><br>

## Object
    Objects Contain Data and Behavior:
        Object-oriented programs are made up of objects. An object packages both data and the procedures that operate on that data. The procedures are typically called methods or operations



<!--------------------------------------------------------------------------------- Encapsulation -->
<br><br>

## Encapsulation
    Encapsulation That Hides Implementation Details:s
        which means that the implementation details of an object aren’t accessible to code using that object

```rust
pub struct AveragedCollection 
{
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) 
    {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> 
    {
        let result = self.list.pop();
        match result 
        {
            Some(value) => 
            {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 
    {
        self.average
    }

    fn update_average(&mut self) 
    {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}
```



<!--------------------------------------------------------------------------------- Inheritance -->
<br><br>

## Inheritance

    Inheritance as a Type System and as Code Sharing:
        Inheritance is a mechanism whereby an object can inherit elements from another object’s definition, thus gaining the parent object’s data and behavior without you having to define them again.


```rust

```



<!--------------------------------------------------------------------------------- Polymorphism -->
<br><br>

## Polymorphism
    refers to code that can work with data of multiple types