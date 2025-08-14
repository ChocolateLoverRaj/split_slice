# `split_slice`
[![Crates.io](https://img.shields.io/crates/v/split_slice.svg)](https://crates.io/crates/split_slice)
[![Docs.rs](https://img.shields.io/docsrs/split_slice)](https://docs.rs/split_slice)

A Rust library for accessing `(&[T], &[T])` as if it were a single slice.

## Features
- Very simple to construct
- `len` and `is_empty`
- `impl Debug` shows it as a single list
- `Iterator`
- Pure Rust, no dependencies, works with `no_std`
- No `alloc` used
- `T` does not need to implement `Clone` or `Copy`

## Use Cases
- Ring buffers
- Custom `VecDequeue`
