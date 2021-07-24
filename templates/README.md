# Keycloak Admin REST API

## Legal

Dual-licensed under `MIT` or the [UNLICENSE](http://unlicense.org/).

## Features

Implements [Keycloak Admin REST API version 13](https://www.keycloak.org/docs-api/13.0/rest-api/index.html).

## Usage

Add dependency to Cargo.toml:

```toml
[dependencies]
keycloak = "13"
```

{{ codeblock "rust" ( read_to_str "examples/adduser.rs" ) }}

## Version agreement

If we have `x.y.z` version of `keycloak`, our package version would be `x.y.(z * 100 + v)` there v is a minor
fix version to official `x.y.z` version.

Example: official version `13.0.1` is `13.0.100` for crate version.