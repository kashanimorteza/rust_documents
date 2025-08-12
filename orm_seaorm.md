
# ORM : SeaORM 
    Migration
    Entities
    CRUD



<!--------------------------------------------------------------------------------- Install -->
<br><br>

## Install

Cargo
```bash
cargo add anyhow
cargo add tokio
cargo add sea-orm --no-default-features --features sqlx-mysql,sqlx-postgres,sqlx-sqlite,runtime-async-std-rustls,runtime-tokio-rustls,macros,debug-print,seaography,with-uuid,with-chrono,with-json,with-bigdecimal,with-time
cargo add sea-orm-migration --no-default-features --features "sqlx-mysql,sqlx-postgres,sqlx-sqlite,runtime-tokio-rustls"
cargo install sea-orm-cli --force --no-default-features --features "cli,codegen,sqlx-mysql,sqlx-postgres,sqlx-sqlite,runtime-tokio-rustls,runtime-async-std-rustls,async-std"
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

Status
```bash
sea-orm-cli migrate status
```

example
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
