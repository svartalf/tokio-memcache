# Generic over serde

It would be really nice to make the following:
 * `Client` and `MemcacheCodec` generic over `serde::Serializer` and `serde::Deserializer`
 * `Request` and `Response` key and value fields generic over `serde::Serialize` and `serde::Deserialize`

This pseudo-code might help with an idea:

```rust
let config = Config {
    serializer: serde_json::Serializer::default(),
    deserializer: serde_json::Serializer::default(),
};

let some_data = MyData {
    name: "Jake".to_string(),
    perks: vec!["jumping", "running", "laying down and sleeping"],
}

let future = Client::connect("127.0.0.1:11211".into(), &handle, &config)
    .and_then(|conn| {
        conn.set("what-time-is-it?", some_data, extras::Set::default());
    });
```

If `MyData` struct derives `Serialize + Deserialize`,
if will be automatically encoded into JSON by codec and stored into memcached.
Same way backwards, deserialization should be made automatically on the `conn.get()`.

Unfortunately, I can't achieve it right now, though, `erased_serde` crate
and this https://github.com/serde-rs/serde/issues/644 issue (when resolved)
might help me with that issue. Until then, serde_bincode will be used as default and only encoder/decoder.
