# Catalog index tools

A small rust tool for checking that it is safe to swap catalog
indices.

## Usage

1. Get on the VPN
1. On catalog-qa1, set the app_configs to point to the rebuild solr index
1. Locally, in this repo, run `cargo run`

If the index you want to swap is somewhere different than catalog-qa1, use
the `FUTURE_PROD` environment variable.  For example:

```
FUTURE_PROD=http://localhost:3000 cargo run
```

## Tests

```
cargo test
```

## Formatting

```
cargo fmt
```

## Linting

```
cargo clippy
```
