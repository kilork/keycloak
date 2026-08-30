# AGENTS.md

Rust crate: generated async bindings for the Keycloak Admin REST API. See README.md for usage docs.

## Build & test

```sh
cargo build                                        # default features
cargo test --lib                                   # unit tests
cargo build --features=rc,schemars,multipart,resource-builder   # all extra features (CI runs both)
cargo fmt --all && cargo clippy --all-targets      # style
```

Toolchain pinned in `rust-toolchain.toml`.

Integration tests are the `examples/` binaries run against a live Keycloak. The Keycloak image version is derived from the crate version (see `.github/workflows/integration.yml` for the exact mapping):

```sh
docker run -p 8080:8080 -e KEYCLOAK_ADMIN=admin -e KEYCLOAK_ADMIN_PASSWORD=password \
  quay.io/keycloak/keycloak:<keycloak-version> start-dev &
sleep 40
cargo run --example=adduser
cargo run --example=resource_adduser --features=resource-builder
cargo run --example=importconfig --features=multipart
```

`KEYCLOAK_ADDR` / `KEYCLOAK_USER` / `KEYCLOAK_PASSWORD` env vars override defaults.

## Code generation — do not hand-edit generated files

Input: `api/openapi.json` (official Keycloak OpenAPI spec, compiled into the generator via `include_bytes!`).
Generator: `examples/openapi.rs`, a clap CLI. All subcommands print to stdout; `update.ts` captures output and writes files.

| Subcommand | Output |
|---|---|
| `cargo run --example=openapi -- types` | `src/types.rs` — one serde struct per component schema |
| `... -- rest --tag <tag>` / `--no-tag` | `src/rest/generated_rest/<tag>.rs` — low-level `KeycloakAdmin` methods |
| `... -- methods --tag <tag>` / `--no-tag` | `src/resource/<tag>.rs` — `KeycloakRealmAdmin` fluent layer |
| `... -- tags [cargo\|kebab\|mod-resource\|mod-rest]` | tag list / Cargo feature flags / `mod.rs` declarations |
| `... -- specs` | dump parsed spec (debugging) |

Untagged operations land in `other_methods.rs` (both layers). Generated files: `src/types.rs`, `src/rest/generated_rest/**`, `src/resource/**`.

### Naming rules

- Method name = path, `{param}` replaced by `with_<param>` (except `realm`), plus HTTP verb, all snake_case. Low-level `KeycloakAdmin` methods keep the full name: `GET /admin/realms/{realm}/users` → `realm_users_get`, `.../{user_id}/role-mappings` → `realm_users_with_user_id_role_mappings_get`.
- Realm layer (`KeycloakRealmAdmin`) uses the same names with the `realm_` prefix stripped: `users_get()`, `users_with_user_id_delete(user_id)`.
- Params and struct fields: snake_case; Rust reserved words get a trailing `_` (`type` → `type_`).

### Type mapping

- Struct fields use the `TypeString` / `TypeVec<T>` / `TypeMap<K, V>` / `TypeValue` aliases so `rc-*` features can swap in `Arc`-based types; generated code never hardcodes the concrete type.
- Required params → refs (`&str`, `&[T]`); optional params and bodies → `Option<...>` / owned.
- Untyped/free-form JSON → `serde_json::Value`.
- Structs: `#[skip_serializing_none]`, derives `Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize` (+ `JsonSchema` under `schemars`). If a struct has more camelCase than snake_case fields it gets `rename_all = "camelCase"`, otherwise per-field `#[serde(rename)]` as needed.
- Response = the `200` (or first `2xx`) entry, parsed as json/text/bytes/form; no 2xx → `DefaultResponse`.
- Fluent layer: operations with optional params get a struct named after the full method (e.g. `RealmUsersGet`) with required params as fields + `<Name>Args` (optional params, `Default`) + `IntoFuture` (so `.await` works directly) + `builder`-feature `Builder` impl (`.username(v)` etc.). Methods with no optional params are plain `-> impl Future` fns.

### Tuning: `examples/openapi.patch.toml`

The generator is compiled with `include_str!` on this file — edit it, rebuild the example, regenerate. Mapping kinds:

- `[path."<path>:<method>:<param>"]` — override a parameter type (`body` for the request body).
- `[path."<path>:<method>:"]` — override the response type; optional `method` (replaces the reqwest body call) and `convert` (post-parse conversion) fields.
- `[type."<Struct>:<field>"]` — override a struct field type.

Each `path` mapping records `from_type` = the type inferred from the spec. On every generation run the generator re-checks: if the spec changed and the freshly inferred type now equals `rust_type`, the mapping is **deleted automatically** (the file self-heals across version bumps); if the inference differs from `from_type`, it prints a `warn:` that the spec changed and the mapping may be stale. Inspect what the generator sees with `cargo run --example=openapi -- specs`.

### Hand-written (safe to edit)

`src/error.rs`, `src/builder.rs`, `src/prelude.rs`, `src/rest/mod.rs`, `src/rest/manual_rest.rs` (methods the spec can't express, e.g. multipart import), `src/rest/url_enc.rs`, `src/rest/default_response.rs`.

### Rendered docs

`README.md` and the doc-header of `src/lib.rs` (module declarations below it are hand-written) are rendered from `templates/` via `handlebars-magic templates .` using env vars `KEYCLOAK_VERSION`, `KEYCLOAK_RUST_VERSION`, `KEYCLOAK_RUST_MAJOR_VERSION`; the usage example is embedded from `examples/resource_adduser.rs`.

## Releases — update.ts

Releases and version bumps go through the interactive deno script `./update.sh` (runs `update.ts`). Requirements: deno, `gh` (authenticated), git, `handlebars-magic`, cargo.

- Two modes: **Generation** (regenerate from a Keycloak version, no git/GitHub actions by default) and **Release** (full flow, all git actions on by default).
- Base version: latest Keycloak (scraped from keycloak.org/documentation), current Cargo.toml version, an existing milestone, or manual.
- Steps, in order, each prompted: create/assign GitHub milestone → bump Cargo.toml → download OpenAPI spec to `api/openapi.json` → render docs (`handlebars-magic`) → regenerate code (first `git checkout`-discards local changes to the generated files, then regenerates, `cargo fmt`, `cargo build`) → release issue → commit → push → release PR → merge → git tag → GitHub release → `cargo publish`.

Do not hand-edit versions: Cargo.toml, README.md, and `src/lib.rs` docs all come from this flow.

## Versioning

Crate `x.y.(z*100+v)` maps to Keycloak `x.y.z` with patch `v` (e.g. `26.6.2` = Keycloak `26.6.0`, fix 2). A patch `v > 0` means a fix release against the same Keycloak version, not a new Keycloak.

## Features

- `tags-<name>`: each OpenAPI tag is a feature flag gating its module; `tags-all` is default.
- `reqwest` = reqwest 0.13 (default); `reqwest12` for 0.12. New reqwest APIs must work on both.
- `multipart` gates extra reqwest multipart methods (see `manual_rest.rs`).
- `resource-builder` gates the builder pattern (`src/builder.rs`).
