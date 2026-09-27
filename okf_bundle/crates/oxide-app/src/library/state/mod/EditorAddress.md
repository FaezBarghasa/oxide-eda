---
okf_version: "0.2"
type: Class
title: EditorAddress
description: Identity for an open Component Preview tab — the lookup key for
resource: crates/oxide-app/src/library/state/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/state/mod/EditorAddress
language: rust
---

# EditorAddress

Identity for an open Component Preview tab — the lookup key for

## Signature

```rust
pub struct EditorAddress
```

## Decorators

- `derive(Debug, Clone, PartialEq, Eq, Hash)`

## Visibility

- `pub`

## Docstring

Identity for an open Component Preview tab — the lookup key for
[`LibraryState::editors`] and the address that preview view closures
clone into messages. Rows live in `tables/<name>.tsv` and are
addressed by `(library_path, table, row_id)`.
[derive(Debug, Clone, PartialEq, Eq, Hash)]

## Methods

- `library_path`
- `table`
- `row_id`

## Source
Lines 48–52 in `crates/oxide-app/src/library/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/library/state/mod.md) |
