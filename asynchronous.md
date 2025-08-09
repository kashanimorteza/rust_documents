# Asynchronous
    All example about Asynchronous
    Futures, async, await




<!--------------------------------------------------------------------------------- Futures -->
<br><br>

## Futures

```rust
extern crate trpl;
use trpl::Html;
fn main() 
{
    let url = String::from("https://www.rust-lang.org");

    trpl::run(async {
        let url = &url;
        match page_title(url).await 
        {
            Some(title) => println!("The title for {url} was {title}"),
            None => println!("{url} had no title"),
        }
    })
}

async fn page_title(url: &str) -> Option<String> 
{
    let response_text = trpl::get(url).await.text().await;
    Html::parse(&response_text).select_first("title").map(|title| title.inner_html())
}
```

Racing Our Two URLs Against Each Other
```rust
use trpl::{Either, Html};
fn main() 
{
    trpl::run(async {
        let url1 = String::from("https://www.rust-lang.org");
        let url2 = String::from("https://git-scm.com/");

        let title_fut_1 = page_title(&url1);
        let title_fut_2 = page_title(&url2);

        let (url, maybe_title) =
            match trpl::race(title_fut_1, title_fut_2).await 
            {
                Either::Left(left) => left,
                Either::Right(right) => right,
            };

        println!("{url} returned first");
        match maybe_title 
        {
            Some(title) => println!("Its page title was: '{title}'"),
            None => println!("It had no title."),
        }
    })
}

async fn page_title(url: &str) -> (&str, Option<String>) 
{
    let response_text = trpl::get(url).await.text().await;
    let title = Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html());
    (url, title)
}
```



<!--------------------------------------------------------------------------------- Concurrency -->
<br><br>

## Concurrency

spawn_task
```rust
extern crate trpl; // required for mdbook test
use std::time::Duration;

fn main() 
{
    trpl::run(async {
        trpl::spawn_task(async {
            for i in 1..10 
            {
                println!("{i} : first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..5 
        {
            println!("{i} : second task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }
    });
}
```

```rust
extern crate trpl; // required for mdbook test

use std::time::Duration;

fn main() 
{
    trpl::run(async {
        let fut1 = async {
            for i in 1..10 
            {
                println!("{i} : first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let fut2 = async {
            for i in 1..5 
            {
                println!("{i} : second task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        trpl::join(fut1, fut2).await;
    });
}
```

```rust

```

```rust

```

```rust

```