---
okf_version: "0.2"
type: Class
title: Raw
description: "Intermediate shape — captures the manifest header fields, the"
resource: crates/oxide-library/src/library_file/codec.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/library_file/codec/Raw
language: rust
---

# Raw

Intermediate shape — captures the manifest header fields, the

## Signature

```rust
struct Raw
```

## Decorators

- `derive(Deserialize)`

## Docstring

Intermediate shape — captures the manifest header fields, the
raw TSV strings, and any `[tables.<name>.column_types]` sidecar
map in one TOML deserialization pass. The split back into
[`SnxlibManifest`] + parsed `tables` happens after we hand the
TSV strings to [`parse_tsv`] and merge the type sidecar.
[derive(Deserialize)]

## Methods

- `format`
- `library_id`
- `library`
- `mode`
- `workflow`
- `users`
- `classes`
- `tables`

## Source
Lines 15–29 in `crates/oxide-library/src/library_file/codec.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [codec](/crates/oxide-library/src/library_file/codec.md) |
