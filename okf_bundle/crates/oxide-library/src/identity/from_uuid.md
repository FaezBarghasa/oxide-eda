---
okf_version: "0.2"
type: Function
title: from_uuid
description: Wrap an existing UUID — used when reading a row back from disk.
resource: crates/oxide-library/src/identity.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/identity/from_uuid
language: rust
---

# from_uuid

Wrap an existing UUID — used when reading a row back from disk.

## Signature

```rust
impl RowId { pub fn from_uuid(uuid: Uuid) -> Self }
```

## Visibility

- `pub`

## Docstring

Wrap an existing UUID — used when reading a row back from disk.

## Source
Lines 21–23 in `crates/oxide-library/src/identity.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [identity](/crates/oxide-library/src/identity.md) |
