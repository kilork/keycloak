#!/bin/sh

deno run --allow-env=KEYCLOAK_RUST_VERSION,KEYCLOAK_VERSION,KEYCLOAK_RUST_MAJOR_VERSION --allow-read=Cargo.toml --allow-write=Cargo.toml,api/openapi.json,src/types.rs,src/rest/generated_rest,src/resource --allow-net=keycloak.org,www.keycloak.org --allow-run=cargo,gh,git,handlebars-magic update.ts