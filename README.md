# `lta-models`
<p align="left">
  <a href="https://github.com/lta-rs/lta-models/blob/main/LICENSE">
    <img src="https://img.shields.io/github/license/lta-rs/lta-models"/>
  </a>
  <a href="https://docs.rs/lta_models/">
    <img src="https://img.shields.io/badge/docs-docs.rs-blue"/>
  </a>
  <a href="https://lta-rs.github.io/lta-models/lta_models/">
    <img src="https://img.shields.io/badge/docs-main--branch-red"/>
  </a>
  <a href="https://github.com/lta-rs/lta-models/actions">
    <img src="https://img.shields.io/github/workflow/status/lta-rs/lta-models/Test%20Rust%20project/main"/>
  </a>
  <a href="https://crates.io/crates/lta_models">
    <img src="https://img.shields.io/crates/v/lta-models"/>
  </a>
</p>

This repository contains the data structures required to interact with LTA's datamall APIs. All data structures implements `Serialize` and `Deserialize`.


## `Cargo.toml` setup
```toml
# extra features available: fastfloat
lta-models = { version = "0.5.0" }
```

## Performance & `fast-float` implementation
Some of the deserialization code _may_ benefit from using the `fastfloat` feature, but during testing the biggest performance improvement can be seen when you swap out the system allocator to something faster like [`mimalloc`](https://github.com/microsoft/mimalloc) or [`jemalloc`](https://github.com/jemalloc/jemalloc)

## How to use this?
You can use this to deserialize LTA's datamall APIs. You can take a look at [lta-rs](https://github.com/lta-rs/lta-rs) for an example.

## Is there anything that I need to do to use `fastfloat`?
Just add the feature to your `Cargo.toml`. You don't have to do anything else.

##  License
lta-models is licensed under MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)