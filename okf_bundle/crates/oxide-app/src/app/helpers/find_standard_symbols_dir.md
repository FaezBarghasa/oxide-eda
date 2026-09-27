---
okf_version: "0.2"
type: Function
title: find_standard_symbols_dir
description: Find the Standard symbol library directory.
resource: crates/oxide-app/src/app/helpers.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/app/helpers/find_standard_symbols_dir
language: rust
---

# find_standard_symbols_dir

Find the Standard symbol library directory.

## Signature

```rust
pub(super) fn find_standard_symbols_dir() -> Option<PathBuf>
```

## Visibility

- `pub(super)`

## Docstring

Find the Standard symbol library directory.

## Source
Lines 8–50 in `crates/oxide-app/src/app/helpers.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [helpers](/crates/oxide-app/src/app/helpers.md) |
| called_by | [new](/crates/oxide-app/src/app/bootstrap/new/new.md) |
