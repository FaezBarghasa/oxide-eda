---
okf_version: "0.2"
type: Function
title: encode_segment
description: Percent-encode a single path segment per RFC 3986.
resource: crates/oxide-library/src/adapters/database.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/adapters/database/encode_segment
language: rust
---

# encode_segment

Percent-encode a single path segment per RFC 3986.

## Signature

```rust
impl DatabaseAdapter { fn encode_segment(s: &str) -> String }
```

## Docstring

Percent-encode a single path segment per RFC 3986.
Used for every user-controlled string that flows into the URL
path (table names, primitive collection names) so a value like
`"Discrete Passives"` or `"resistors?evil"` can't reshape the
request URL.

## Source
Lines 166–177 in `crates/oxide-library/src/adapters/database.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [database](/crates/oxide-library/src/adapters/database.md) |
