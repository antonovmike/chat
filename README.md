# chat
Simple terminal chat

```bash
cargo run --bin receiver
cargo run --bin transmitter
```

![text chat](https://github.com/antonovmike/chat/blob/main/Screenshot.png)

TODO: get rid of last unwrap transmitter/transmitter.rs

```Rust
let serialized = serde_json::to_string(&user_data)
    .unwrap()
    .clone()
    .into_bytes();
```