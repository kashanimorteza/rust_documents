# Orm
    All example about orm



<!--------------------------------------------------------------------------------- Structure -->
<br><br>

## Structure
    migrations, model, schema 

```sql
--migration/post/up.sql
CREATE TABLE posts 
(
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  title VARCHAR NOT NULL,
  family VARCHAR NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 0
)
```

```sql
--migration/post/down.sql
DROP TABLE posts
```

<!--------------------------------------------------------------------------------- Install -->
<br><br>

## Install  
    cargo add diesel
    cargo install diesel_cli --no-default-features --features "sqlite postgres"



<!--------------------------------------------------------------------------------- Setup -->
<br><br>

## Setup
    diesel setup
    diesel migration generate post
    diesel migration run



<!--------------------------------------------------------------------------------- Select -->
<br><br>

### Select



<!--------------------------------------------------------------------------------- Insert -->
<br><br>

### Insert
Simple
```rust
fn main() 
{
    //---------------------Import
    use diesel::prelude::*;

    //---------------------Connection
    dotenvy::dotenv().ok();
    let database_url = "sample.db";
    let connection = &mut  SqliteConnection::establish(&database_url).unwrap_or_else(|e| panic!("Failed to connect, error: {e}"));

    //---------------------Schema
    diesel::table! 
    {
        posts (id) 
        {
            id -> Integer,
            title -> Text,
            family -> Text,
            published -> Bool,
        }
    }

    //---------------------Model
    //-----Select
    #[derive(Queryable, Selectable)]
    #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
    #[diesel(table_name = posts)]
    pub struct ModelSelect 
    {
        pub id: i32,
        pub title: String,
        pub family: String,
        pub published: bool,
    }
    //-----Insert
    #[derive(AsChangeset, Default, Insertable)]
    #[diesel(table_name = posts)]
    pub struct ModelInsert<'a> 
    {
        pub title: &'a str,
        pub family: &'a str,
        pub published: bool,
    }

    //---------------------Function
    pub fn insert(connection: &mut SqliteConnection)-> ModelSelect
    {
        let items = ModelInsert { 
            title:"insert-1", 
            family:"insert_1", 
            published:true 
        };
        
        diesel::insert_into(posts::table)
        .values(&items)
        .returning(ModelSelect::as_returning())
        .get_result(connection)
        .expect("Error saving new post")
    }

    //---------------------Run
    let result = insert(connection);
    println!("Inserted: id={}, title={}, family={}, published={}", result.id, result.title, result.family, result.published);
}
```

Input
```rust
fn main() 
{
    //---------------------Import
    use diesel::prelude::*;

    //---------------------Connection
    dotenvy::dotenv().ok();
    let database_url = "sample.db";
    let connection = &mut  SqliteConnection::establish(&database_url).unwrap_or_else(|e| panic!("Failed to connect, error: {e}"));

    //---------------------Schema
    diesel::table! 
    {
        posts (id) 
        {
            id -> Integer,
            title -> Text,
            family -> Text,
            published -> Bool,
        }
    }

    //---------------------Model
    //-----Select
    #[derive(Queryable, Selectable)]
    #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
    #[diesel(table_name = posts)]
    pub struct ModelSelect 
    {
        pub id: i32,
        pub title: String,
        pub family: String,
        pub published: bool,
    }
    //-----Insert
    #[derive(AsChangeset, Default, Insertable)]
    #[diesel(table_name = posts)]
    pub struct ModelInsert<'a> 
    {
        pub title: &'a str,
        pub family: &'a str,
        pub published: bool,
    }

    //---------------------Function
    pub fn insert(connection: &mut SqliteConnection, items: ModelInsert)-> ModelSelect
    {        
        diesel::insert_into(posts::table)
        .values(&items)
        .returning(ModelSelect::as_returning())
        .get_result(connection)
        .expect("Error saving new post")
    }

    //---------------------Run
    let items = ModelInsert 
    {
        title: "insert-2",
        ..Default::default()
    };
    let result = insert(connection, items);
    println!("Inserted: id={}, title={}, family={}, published={}", result.id, result.title, result.family, result.published);
}
```

Generic
```rust
fn insert_3() 
{
    //---------------------Import
    use diesel::prelude::*;

    //---------------------Connection
    dotenvy::dotenv().ok();
    let database_url = "sample.db";
    let connection = &mut  SqliteConnection::establish(&database_url).unwrap_or_else(|e| panic!("Failed to connect, error: {e}"));

    //---------------------Schema
    //-----posts
    diesel::table! 
    {
        posts (id) 
        {
            id -> Integer,
            title -> Text,
            family -> Text,
            published -> Bool,
        }
    }
    //-----users
    diesel::table! 
    {
        users (id) 
        {
            id -> Integer,
            username -> Text,
            code -> Text,
            published -> Bool,
        }
    }

    //---------------------Model
    //--------------Post
    //-----Select
    #[derive(Queryable, Selectable)]
    #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
    #[diesel(table_name = posts)]
    pub struct ModelSelectPosts 
    {
        pub id: i32,
        pub title: String,
        pub family: String,
        pub published: bool,
    }
    //-----Insert
    #[derive(AsChangeset, Default, Insertable)]
    #[diesel(table_name = posts)]
    pub struct ModelInsertPosts<'a> 
    {
        pub title: &'a str,
        pub family: &'a str,
        pub published: bool,
    }
    //--------------User
    //-----Select
    #[derive(Queryable, Selectable)]
    #[diesel(check_for_backend(diesel::sqlite::Sqlite))]
    #[diesel(table_name = users)]
    pub struct ModelSelectUsers 
    {
        pub id: i32,
        pub username: String,
        pub code: String,
        pub published: bool,
    }
    //-----Insert
    #[derive(AsChangeset, Default, Insertable)]
    #[diesel(table_name = users)]
    pub struct ModelInsertUsers<'a> 
    {
        pub username: &'a str,
        pub code: &'a str,
        pub published: bool,
    }

        //---------------------Function
        pub fn insert(connection: &mut SqliteConnection, items: ModelInsertPosts)-> ModelSelectPosts
        {        
            diesel::insert_into(posts::table)
            .values(&items)
            .returning(ModelSelectPosts::as_returning())
            .get_result(connection)
            .expect("Error saving new post")
        }

        //---------------------Run
        let items = ModelInsertPosts 
        {
            title: "insert-2",
            ..Default::default()
        };
        let result = insert(connection, items);
        println!("Inserted: id={}, title={}, family={}, published={}", result.id, result.title, result.family, result.published);
}
```


<!--------------------------------------------------------------------------------- Update -->
<br><br>

### Update



<!--------------------------------------------------------------------------------- Delete -->
<br><br>

### Delete
