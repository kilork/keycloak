# Keycloak Admin REST API

## Legal

Dual-licensed under `MIT` or the [UNLICENSE](http://unlicense.org/).

## Features

Implements [Keycloak Admin REST API version {{ env_var "KEYCLOAK_VERSION" }}](https://www.keycloak.org/docs-api/{{ env_var "KEYCLOAK_VERSION" }}/rest-api/index.html).

## Usage

Requires Rust version >= `1.58.0`.

Add dependency to Cargo.toml:

```toml
[dependencies]
keycloak = "{{ env_var "KEYCLOAK_RUST_MAJOR_VERSION" }}"
```

If you are using new [Quarkus distribution of Keycloak](https://www.keycloak.org/migration/migrating-to-quarkus) enable `quarkus` feature:

```toml
[dependencies]
keycloak = { version = "{{ env_var "KEYCLOAK_RUST_MAJOR_VERSION" }}", features = [ "quarkus" ] }
```

{{ codeblock "rust" ( read_to_str "examples/adduser.rs" ) }}

## Version agreement

If we have `x.y.z` version of `keycloak`, our package version would be `x.y.(z * 100 + v)` there v is a minor
fix version to official `x.y.z` version.

Example: official version `13.0.1` is `13.0.100` for crate version. `13.0.102` means keycloak version `13.0.1` and minor fix version `2`.