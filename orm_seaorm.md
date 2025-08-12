
# ORM : SeaORM 
    All example about SeaORM



<!--------------------------------------------------------------------------------- Structure -->
<br><br>

## Structure
    Migration
    Entities
    CRUD



<!--------------------------------------------------------------------------------- Install -->
<br><br>

## Install

Cargo
```bash
cargo add tokio
cargo add anyhow
cargo add sea-orm --no-default-features features sqlx-mysql,sqlx-postgres,sqlx-sqlite,runtime-async-std-rustls,runtime-tokio-rustls,macros,debug-print,seaography,with-uuid,with-chrono,with-json,with-bigdecimal,with-time
cargo add sea-orm-migration --no-default-features --features sqlx-mysql,sqlx-postgres,sqlx-sqlite,runtime-tokio-rustls
cargo install sea-orm-cli --force --no-default-features --features sqlx-mysql,sqlx-postgres,sqlx-sqlite,runtime-tokio-rustls
```

Dependencies
```toml
[dependencies]
anyhow = "1.0.98"
dotenvy = "0.15.7"
tokio = { version = "1", features = ["full"] }
sea-orm = { version = "1.1.14", default-features = false, features = [
    "sqlx-mysql",
    "sqlx-postgres",
    "sqlx-sqlite",
    "runtime-async-std-rustls",
    "runtime-tokio-rustls",
    "macros",
    "debug-print",
    "seaography",
    "with-uuid",
    "with-chrono",
    "with-json",
    "with-bigdecimal",
    "with-time"
] }
sea-orm-migration = { version = "1.1.14", default-features = false, features = [
    "sqlx-mysql",
    "sqlx-postgres",
    "sqlx-sqlite",
    "runtime-tokio-rustls"
] }
```

Build
```bash
cargo run
```

ENV
```bash
cat << 'EOF' > .env
#---------------------------------------------------------------------------------DATABASE
#----------------------------------------SQLITE
DATABASE_SQLITE_URL=sqlite://db.sqlite
#DATABASE_SQLITE_URL=sqlite://db.sqlite?mode=rwc
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
Generate
```bash
sea-orm-cli migrate generate create_users_table
```
Status
```bash
sea-orm-cli migrate status
```




