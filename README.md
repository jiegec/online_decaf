# online_decaf

Run Decaf compiler online. Based on jiegec/decaf-rs which is based on decaf-lang/decaf-rs. Using yew for frontend Rust framework.

## How to run

```shell
cargo install cargo-web
cargo web build --bin main
cargo web build --bin worker
cd static
caddy
```

## How to deploy

```shell
cargo web build --bin main --release
cargo web build --bin worker --release
```

See .github/workflows/rust.yml for more.
