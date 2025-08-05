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



<!--------------------------------------------------------------------------------- Update -->
<br><br>

### Update



<!--------------------------------------------------------------------------------- Delete -->
<br><br>

### Delete
