
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
cargo add sea-orm
cargo add tokio
cargo add anyhow
cargo install sea-orm-cli
```

Dependencies
```toml
[dependencies]
sea-orm = { version = "0.12", features = ["sqlx-postgres", "sqlx-mysql", "sqlx-sqlite", "runtime-tokio-native-tls"] }
tokio = { version = "1", features = ["full"] }
anyhow = "1.0.98"
dotenvy = "0.15.7"
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
DATABASE_SQLITE_URL=sample.db

#----------------------------------------POSTGRESQL
DATABASE_POSTGRESQL_HOST=localhost
DATABASE_POSTGRESQL_USER=raspberrypi_api
DATABASE_POSTGRESQL_PASS=123456
DATABASE_POSTGRESQL_DB=raspberrypi_api
DATABASE_POSTGRESQL_URL=postgres://${DATABASE_POSTGRESQL_USER}:${DATABASE_POSTGRESQL_PASS}@${DATABASE_POSTGRESQL_HOST}/${DATABASE_POSTGRESQL_DB}
EOF
```



<!--------------------------------------------------------------------------------- Migration -->
<br><br>

## Migration
