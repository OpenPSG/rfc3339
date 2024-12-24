# rfc3339

A Portable RFC3339 Timestamp Formatter for Rust. With support for `no_std` and
`alloc` free environments.

## Usage

```rust
use rfc3339::format_unix;

fn main() {
    let timestamp = format_unix(1445470140, 0);
    println!("{}", timestamp); // 2015-10-21T23:29:00.000000Z
}
```

## License

Licensed under the Mozilla Public License, version 2.0 ([LICENSE](./LICENSE)).