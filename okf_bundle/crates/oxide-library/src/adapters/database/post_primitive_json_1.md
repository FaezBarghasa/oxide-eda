---
okf_version: "0.2"
type: Function
title: post_primitive_json
description: "Generic POST primitive JSON to `/{collection}` with the supplied"
resource: crates/oxide-library/src/adapters/database.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapters/database/post_primitive_json_1
language: rust
---

# post_primitive_json

Generic POST primitive JSON to `/{collection}` with the supplied

## Signature

```rust
fn post_primitive_json(
        &self,
        collection: &str,
        body: &T,
        message: &str,
    ) -> Result<(), LibraryError>
```

## Type Parameters

- `T: Serialize`

## Docstring

Generic POST primitive JSON to `/{collection}` with the supplied
commit message in the `x-oxide-message` header.

## Source
Lines 215–238 in `crates/oxide-library/src/adapters/database.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [database](/crates/oxide-library/src/adapters/database.md) |
