
# ORM : SeaORM 
    Migration
    Entity
    CRUD



<!--------------------------------------------------------------------------------- Install -->
<br><br>

## Install

Cargo
```bash
cargo add anyhow
cargo add tokio --features macros,rt-multi-thread
cargo add tokio --features full
cargo add sea-orm --no-default-features --features sqlx-mysql,sqlx-postgres,sqlx-sqlite,runtime-async-std-rustls,runtime-tokio-rustls,macros,debug-print,seaography,with-uuid,with-chrono,with-json,with-bigdecimal,with-time
cargo install sea-orm-cli --force --no-default-features --features "cli,codegen,sqlx-mysql,sqlx-postgres,sqlx-sqlite,runtime-tokio-rustls,runtime-async-std-rustls,async-std"
cargo add sea-orm-migration --no-default-features --features "sqlx-mysql,sqlx-postgres,sqlx-sqlite,runtime-tokio-rustls"
```

ENV
```bash
cat << 'EOF' > .env
#---------------------------------------------------------------------------------DATABASE
#----------------------------------------SQLITE
DATABASE_SQLITE_URL=sqlite://db.sqlite?mode=rwc
#DATABASE_SQLITE_URL=sqlite::memory:

#----------------------------------------POSTGRESQL
DATABASE_POSTGRESQL_HOST=localhost
DATABASE_POSTGRESQL_USER=raspberrypi
DATABASE_POSTGRESQL_PASS=123456
DATABASE_POSTGRESQL_DB=raspberrypi
DATABASE_POSTGRESQL_URL=postgres://${DATABASE_POSTGRESQL_USER}:${DATABASE_POSTGRESQL_PASS}@${DATABASE_POSTGRESQL_HOST}/${DATABASE_POSTGRESQL_DB}

#----------------------------------------Main
DATABASE_URL=${DATABASE_SQLITE_URL}
#DATABASE_URL=${DATABASE_POSTGRESQL_URL}
EOF
```


<!--------------------------------------------------------------------------------- Migration -->
<br><br>

## Migration

Init
```bash
sea-orm-cli migrate init 
```

./migration/cargo.toml
```toml
[dependencies.sea-orm-migration]
version = "1.1.14"
features = ["sqlx-mysql","sqlx-postgres","sqlx-sqlite","runtime-tokio-rustls"]
```

Create migration
```bash
sea-orm-cli migrate generate create_device_command
```

Status
```bash
sea-orm-cli migrate up
```

Structure 1
```bash
sea-orm-cli migrate status
```

Structure 2
```rust
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Post::Table)
                    .if_not_exists()
                    .col(pk_auto(Post::Id))
                    .col(string(Post::Title))
                    .col(string(Post::Text))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(Post::Table)
                    .to_owned()
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Post {
    Table,
    Id,
    Title,
    Text,
}
```

```rust
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
            CREATE TABLE device_command (
                id INTEGER PRIMARY KEY,
                device_id INTEGER NOT NULL,
                name VARCHAR,
                value_from INTEGER,
                value_to INTEGER,
                delay INTEGER,
                description VARCHAR,
                reload BOOLEAN,
                enable BOOLEAN,
                type VARCHAR(7) NOT NULL
            );
        "#;
        manager.get_connection().execute_unprepared(sql).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.get_connection().execute_unprepared("DROP TABLE device_command;").await?;
        Ok(())
    }
}
```



<!--------------------------------------------------------------------------------- Entity -->
<br><br>

## Entity

Create

    sea-orm-cli generate entity -u postgres://postgres:123456@192.168.64.7:5432/raspberrypi -o src/entity
    sea-orm-cli generate entity -u sqlite://db.sqlite -o src/entity



<!--------------------------------------------------------------------------------- CRUD -->
<br><br>

## CRUD
Select
    ss