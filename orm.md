# Orm
    All example about orm



<!--------------------------------------------------------------------------------- aaaa -->
<br><br>

## Install  
    cargo add diesel
    cargo add dotenvy
    cargo install diesel_cli --no-default-features --features "sqlite postgres"



<!--------------------------------------------------------------------------------- Setup -->
<br><br>

## Setup
    diesel setup
    diesel migration generate post
    diesel migration run



<!--------------------------------------------------------------------------------- Connection -->
<br><br>

## Connection 
    echo DATABASE_SQLITE_URL=sample.db > .env 
    echo DATABASE_PGSQL_URL=postgres://raspberrypi_api:morteza123456@127.0.0.1/raspberrypi_api >> .env
    echo DATABASE_URL=sample.db >> .env 