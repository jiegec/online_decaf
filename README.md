# online_decaf

Run Decaf compiler online. Based on MashPlant/tacvm. Using yew for frontend Rust framework.

## How to run

```
cargo install cargo-web
cargo web build --bin main
cargo web build --bin worker
cd static
caddy
```

## How to deploy

```
cargo web build --bin main --release
cargo web build --bin worker --release
```