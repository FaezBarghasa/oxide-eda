---
okf_version: "0.2"
type: Function
title: read_missing_file_is_empty
description: Empty file (no header) reads back as empty vec.
resource: crates/oxide-library/src/tables.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/tables/read_missing_file_is_empty
language: rust
---

# read_missing_file_is_empty

Empty file (no header) reads back as empty vec.

## Signature

```rust
fn read_missing_file_is_empty()
```

## Decorators

- `test`

## Docstring

Empty file (no header) reads back as empty vec.
[test]

## Source
Lines 602–607 in `crates/oxide-library/src/tables.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tables](/crates/oxide-library/src/tables.md) |
| calls | [read_table](/crates/oxide-library/src/tables/read_table.md) |
