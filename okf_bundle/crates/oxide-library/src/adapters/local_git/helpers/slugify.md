---
okf_version: "0.2"
type: Function
title: slugify
description: Slugify a human-facing name into a safe filename component.
resource: crates/oxide-library/src/adapters/local_git/helpers.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/adapters/local_git/helpers/slugify
language: rust
---

# slugify

Slugify a human-facing name into a safe filename component.

## Signature

```rust
pub(super) fn slugify(name: &str) -> String
```

## Visibility

- `pub(super)`

## Docstring

Slugify a human-facing name into a safe filename component.
Lowercased, ASCII-only, runs of non-alphanumeric chars collapsed to
`-`. Empty result falls back to `"untitled"`.

## Source
Lines 179–199 in `crates/oxide-library/src/adapters/local_git/helpers.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [helpers](/crates/oxide-library/src/adapters/local_git/helpers.md) |
| called_by | [fresh_symbol_file_path](/crates/oxide-library/src/adapters/local_git/primitives/fresh_symbol_file_path.md) |
