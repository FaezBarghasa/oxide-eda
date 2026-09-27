---
okf_version: "0.2"
type: Function
title: with_token
description: Explicit bearer-token + holder constructor.
resource: crates/oxide-library/src/adapters/database.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapters/database/with_token_1
language: rust
---

# with_token

Explicit bearer-token + holder constructor.

## Signature

```rust
pub fn with_token(
        url: impl Into<String>,
        token: impl Into<String>,
        holder: impl Into<String>,
    ) -> Result<Self, LibraryError>
```

## Visibility

- `pub`

## Docstring

Explicit bearer-token + holder constructor.

## Source
Lines 107–145 in `crates/oxide-library/src/adapters/database.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [database](/crates/oxide-library/src/adapters/database.md) |
