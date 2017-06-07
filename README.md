
# MinBinaryHeap

The library provides a (binary) min-heap data structure that currently has some of the functionality required to create a min-priority queue of types that form total order.

[![Build Status](https://travis-ci.org/MrKonstantinT/min-binary-heap.svg?branch=master)](https://travis-ci.org/MrKonstantinT/min-binary-heap)

## Documentation

To view project documentation first build it:

```
$ cargo doc
```

and then open the following file with a web browser:

```
target/doc/min_binary_heap/index.html
```

## Usage

Add this entry under `Cargo.toml` `dependencies` section name:

```toml
[dependencies]
min_binary_heap = { git = "https://github.com/MrKonstantinT/min-binary-heap" }
```

and the following to your crate root:

```rust
extern crate min_binary_heap;
```

## License

See the [LICENSE](LICENSE.md) file for license rights and limitations (MIT).
