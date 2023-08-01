# About

This crate serves as the API for our partitioning solution. It uses [`local-robot-map`][lrm] as a library crate, upon which it implements most of the code specific for partitioning.

# Structure

The [`src/main.rs`](./src/main.rs) file serves as the entry point. All API handlers are specified here; currently using the [`axum`](https://crates.io/crates/axum) web framework.

Handling of the incoming HTTP requests is done using functions provided by [`src/polygon_handler.rs`](./src/polygon_handler.rs)

The various partitioning algorithms and functions are provided by [`src/partition_schemes.rs`](./src/partition_schemes.rs)

# Running the application

The [`local-robot-map`][lrm] library crate must be available in the same directory where this crate lives. Please refer to the [`Cargo.toml`](./Cargo.toml) file.

Once set up, you can simply run the application using `cargo run --release`.

```bash
$ cargo run --release
    Finished dev [unoptimized + debuginfo] target(s) in 0.09s
     Running `target/debug/partition-api`
Serving at 0.0.0.0:8000 ...
```

Note that the `--release` flag causes an overflow related to a recursion limit. The bug is likely related to this issue: <https://github.com/rust-lang/rust/issues/110475>. One can either not use the `--release` flag and sacrifice performance, or one can use an older version of Rust (i.e. version 1.69 or maybe even 1.70).

[lrm]: https://github.com/ISM-Thesis-MultiRobot-Partitioning/local-robot-map
