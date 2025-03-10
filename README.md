# Axum rust-web-app


## Starting the DB

```sh
# Start postgresql server docker image:
docker run --rm --name pg -p 5432:5432  -e POSTGRES_PASSWORD=welcome  postgres:15

# (optional) To have a psql terminal on pg. 
# In another terminal (tab) run psql:
docker exec -it -u postgres pg psql

# (optional) For pg to print all sql statements.
# In psql command line started above.
ALTER DATABASE postgres SET log_statement = 'all';
```

## Dev (REPL)

> NOTE: Install cargo watch with `cargo install cargo-watch`.

```sh
# Terminal 1 - To run the server.
cargo watch -q -c -w src/ -x "run"

# Terminal 2 - To run the quick_dev.
cargo watch -q -c -w examples/ -x "run --example quick_dev"
```


## Test

```sh
# Turn of the server & quick_dev
cargo watch -q -c -x "test test_create_ok -- --nocapture"

```




## Misc
```sh
# list all the databases
\l


#select the database
\c mydatabase


#list all tables
\dt
```
