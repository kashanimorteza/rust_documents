# orm_1
    this is orm_1



<!--------------------------------------------------------------------------------- Database -->
<br><br>

# Database
<!------------------------- Install -->
Install
```bash
sh -c 'echo "deb http://apt.postgresql.org/pub/repos/apt $(lsb_release -cs)-pgdg main" > /etc/apt/sources.list.d/pgdg.list'
wget --quiet -O - https://www.postgresql.org/media/keys/ACCC4CF8.asc | sudo apt-key add -
apt update
apt -y install postgresql
```
<!------------------------- Config -->
Config
```bash
sed -i "s/#listen_addresses = 'localhost'/listen_addresses = '0.0.0.0'/" /etc/postgresql/17/main/postgresql.conf
sed -i "s/max_connections = 100/max_connections = 200/" /etc/postgresql/17/main/postgresql.conf
echo "host all all 0.0.0.0/0 md5" >> /etc/postgresql/17/main/pg_hba.conf
```
<!------------------------- Service -->
Service
```bash
sudo systemctl restart postgresql
sudo systemctl status postgresql
```
<!------------------------- Database -->
Database
```bash
PGPASSWORD='123456' psql -h 192.168.64.9 -U postgres -c "DROP DATABASE db_orm_1;"
PGPASSWORD='123456' psql -h 192.168.64.9 -U postgres -c "CREATE DATABASE db_orm_1"
PGPASSWORD='123456' psql -U postgres -h 192.168.64.9 -d db_orm_1 -f db_postgres.sql
```



<!--------------------------------------------------------------------------------- ORM -->
<br><br>

# ORM
<!------------------------- Entity -->
Install
```bash
cargo add dotenvy
cargo add tokio --features full
cargo add sea-orm --no-default-features --features sqlx-mysql,sqlx-postgres,sqlx-sqlite,runtime-async-std-rustls,runtime-tokio-rustls,macros,debug-print,seaography,with-uuid,with-chrono,with-json,with-bigdecimal,with-time
```
<!------------------------- Entity -->
Entity
```bash
sea-orm-cli generate entity -u postgres://postgres:123456@192.168.64.9:5432/db_orm_1 -o src/models
```



<!--------------------------------------------------------------------------------- Run -->
<br><br>

# Run
```bash
cd orm_1
cargo run
```