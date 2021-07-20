# Keycloak Admin REST API

## Legal

Dual-licensed under `MIT` or the [UNLICENSE](http://unlicense.org/).

## Features

Implements [Keycloak Admin REST API version 12](https://www.keycloak.org/docs-api/12.0/rest-api/index.html).

## Usage

Add dependency to Cargo.toml:

```toml
[dependencies]
keycloak = "12"
```

{{ codeblock "rust" ( read_to_str "examples/adduser.rs" ) }}
