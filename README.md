# wasm-logger

A logger that prints all messages with a readable output format, to either the
browser console or stdout, depending on the build target.

The output format is based on the format used by [Supervisord](http://supervisord.org/).

Usage
-----

```rust
#[macro_use]
extern crate log;
extern crate wasm_logger;

fn main() {
    wasm_logger::init().unwrap();

    warn!("This is an example message.");
}
```

This outputs:

```
2015-02-24 01:05:20 WARN [logging_example] This is an example message.
```

You can run the above example with:

```bash
cargo run --example init
```

Licence
-------

`wasm_logger` is licenced under the [MIT Licence](http://opensource.org/licenses/MIT).

Authors
-------

Written by [Sam Clements](sam@borntyping.co.uk).
