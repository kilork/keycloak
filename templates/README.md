# Keycloak Admin REST API

## Legal

Dual-licensed under `MIT` or the [UNLICENSE](http://unlicense.org/).

## Features

Implements [Keycloak Admin REST API version {{ env_var "KEYCLOAK_VERSION" }}](https://www.keycloak.org/docs-api/{{ env_var "KEYCLOAK_VERSION" }}/rest-api/index.html).

### Feature flags

Default flags: `tags-all`.

- `rc`: use `Arc` for deserialization.
- `schemars`: add [schemars](https://crates.io/crates/schemars) support.
- `multipart`: add multipart support to reqwest, enabling extra methods in API.
- `tags-all`: activate all tags (resource groups) in REST API, it is default behavior. Disable default features and use individual `tag-xxx` features to activate only required resource groups. For a full list reference the [Cargo.toml](Cargo.toml).

## Usage

Requires Rust version >= `1.87.0`.

Add dependency to Cargo.toml:

```toml
[dependencies]
keycloak = "~{{ env_var "KEYCLOAK_RUST_MAJOR_VERSION" }}"
```

{{ codeblock "rust" ( from "#[tokio::main]" ( read_to_str "examples/resource_adduser.rs" ) ) }}

## Version agreement

If we have `x.y.z` version of `keycloak`, our package version would be `x.y.(z * 100 + v)` there v is a minor
fix version to official `x.y.z` version.

Example: official version `13.0.1` is `13.0.100` for crate version. `13.0.102` means keycloak version `13.0.1` and minor fix version `2`.

## Update

To update current version use provided [update.ts](./update.ts) `deno` script:

```sh
deno run --allow-env=KEYCLOAK_RUST_VERSION,KEYCLOAK_VERSION,KEYCLOAK_RUST_MAJOR_VERSION --allow-read=Cargo.toml --allow-write=Cargo.toml,api/openapi.json,src/types.rs,src/rest/generated_rest.rs --allow-net=keycloak.org,www.keycloak.org --allow-run=cargo,gh,git,handlebars-magic update.ts
```
