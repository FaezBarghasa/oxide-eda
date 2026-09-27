---
okf_version: "0.2"
type: Function
title: new
description: "Construct from a manifest. The manifest's `auth` field is treated as"
resource: crates/oxide-library/src/adapters/database.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapters/database/new_1
language: rust
---

# new

Construct from a manifest. The manifest's `auth` field is treated as

## Signature

```rust
pub fn new(manifest: Manifest) -> Result<Self, LibraryError>
```

## Visibility

- `pub`

## Docstring

Construct from a manifest. The manifest's `auth` field is treated as
the bearer token; the holder is derived from it.

## Source
Lines 54–88 in `crates/oxide-library/src/adapters/database.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [database](/crates/oxide-library/src/adapters/database.md) |
