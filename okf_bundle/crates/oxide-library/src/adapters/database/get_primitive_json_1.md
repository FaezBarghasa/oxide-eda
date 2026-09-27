---
okf_version: "0.2"
type: Function
title: get_primitive_json
description: "Generic GET → JSON for a primitive at `/{collection}/{uuid}`."
resource: crates/oxide-library/src/adapters/database.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapters/database/get_primitive_json_1
language: rust
---

# get_primitive_json

Generic GET → JSON for a primitive at `/{collection}/{uuid}`.

## Signature

```rust
fn get_primitive_json(
        &self,
        collection: &str,
        uuid: Uuid,
        kind_label: &str,
    ) -> Result<T, LibraryError>
```

## Type Parameters

- `T: DeserializeOwned`

## Docstring

Generic GET → JSON for a primitive at `/{collection}/{uuid}`.
`collection` is percent-encoded; `uuid` is hex (Display) so it
is already URL-safe.

## Source
Lines 191–211 in `crates/oxide-library/src/adapters/database.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [database](/crates/oxide-library/src/adapters/database.md) |
