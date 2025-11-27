# POC - Real time Voting System

# Challenge

Implement: Real time Voting System

## Solution proposal

## Do:

- [ ] Implementation
- [ ] Unit tests
- [ ] Performance Test / Benchmarks
- [ ] Proper Documentation
- [ ] Expose Solution via REST API


## How to install

1. Install Rust:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Create projetct

```
cargo new hello-rust
```

3. Build/install dependencies

```
cargo build
```


## How to run the app w

1. Run project

```
cargo run
```


## How to Call API

1. Vote

```
curl --location 'http://localhost:8080/vote' \
--header 'Content-Type: application/json' \
--data '{
    "poll_id": 1,
    "option_id": 1,
    "voter_id": "7559d194-a50f-45e5-8048-c9ff8d139d7c"
}'
```


## How to run tests

### Unit Tests

1. Ensure Rust is installed on your host
2. Ensure the app and redis are running
3. Execute the unity tests

```
cargo test
```


## Other commands

### When you need to update the libraries version

```
cargo update -p redis
```



## References

- https://rust-lang.org/pt-BR/learn/get-started/
- https://crates.io/crates/redis (Repository central)
