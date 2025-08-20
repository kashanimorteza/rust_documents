# raspberrypi_iot_core_rust
    this is raspberrypi_iot_core_rust

<!--------------------------------------------------------------------------------- Source -->
<br><br>

## Source
```bash
git clone git@github.com:kashanimorteza/raspberrypi_iot_core_rust.git
cd raspberrypi_iot_core_rust
```

<!--------------------------------------------------------------------------------- Database -->
<br><br>

## Database
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
PGPASSWORD='123456' psql -h 192.168.64.9 -U postgres -c "DROP DATABASE raspberrypi;"
PGPASSWORD='123456' psql -h 192.168.64.9 -U postgres -c "CREATE DATABASE raspberrypi"
PGPASSWORD='123456' psql -U postgres -h 192.168.64.9 -d raspberrypi -f db_postgres.sql
```



<!--------------------------------------------------------------------------------- ORM -->
<br><br>

## ORM
<!------------------------- Entity -->
Entity
```bash
sea-orm-cli generate entity -u postgres://postgres:123456@192.168.64.9:5432/raspberrypi -o ./src/orm/model
```



<!--------------------------------------------------------------------------------- API -->
<br><br>

## API

**Add samples:** `cargo run -- --add-samples`
**Running the Server:** `cargo run`
**Get All Users:** `curl -X GET http://localhost:3000/users/items`
**Get User by ID:** `curl -X GET http://localhost:3000/users/1`
**Add New User:** 
```bash
curl -X POST http://localhost:3000/users/add \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Jane Smith",
    "username": "janesmith",
    "password": "securepass123",
    "key": "api-key-jane",
    "email": "jane@example.com",
    "phone": "+987654321",
    "tg_id": "tg-jane",
    "enable": true
  }'
```
**Update User:** 
```bash
curl -X PUT http://localhost:3000/users/update \
  -H "Content-Type: application/json" \
  -d '{
    "id": 19,
    "name": "vvvvvvv",
    "username": "janesmith_updated",
    "password": "newpassword123",
    "key": "api-key-jane-updated",
    "email": "jane.updated@example.com",
    "phone": "+987654321",
    "tg_id": "tg-jane-updated",
    "enable": true
  }'
```
**Delete User:** `curl -X DELETE http://localhost:3000/users/delete/1`
**Disable User:** `curl -X GET http://localhost:3000/users/disable/1`
**Enable User:** `curl -X GET http://localhost:3000/users/enable/1`