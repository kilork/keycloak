#!/bin/sh

git checkout -- src/types.rs src/rest/rest.rs
cargo run --example gen -- types > src/types2.rs
cargo run --example gen -- rest > src/rest/rest2.rs
mv src/types2.rs src/types.rs
mv src/rest/rest2.rs src/rest/rest.rs
cargo fmt
cargo build