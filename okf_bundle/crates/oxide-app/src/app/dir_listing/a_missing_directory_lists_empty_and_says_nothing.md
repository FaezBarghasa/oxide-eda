---
okf_version: "0.2"
type: Function
title: a_missing_directory_lists_empty_and_says_nothing
description: "[test]"
resource: crates/oxide-app/src/app/dir_listing.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dir_listing/a_missing_directory_lists_empty_and_says_nothing
language: rust
---

# a_missing_directory_lists_empty_and_says_nothing

[test]

## Signature

```rust
fn a_missing_directory_lists_empty_and_says_nothing()
```

## Decorators

- `test`

## Docstring

[test]

## Source
Lines 104–118 in `crates/oxide-app/src/app/dir_listing.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [dir_listing](/crates/oxide-app/src/app/dir_listing.md) |
| calls | [unique_dir](/crates/oxide-app/src/app/dir_listing/unique_dir.md) |
| calls | [init_logging](/crates/oxide-app/src/diagnostics/init_logging.md) |
| calls | [list_dir_or_report](/crates/oxide-app/src/app/dir_listing/list_dir_or_report.md) |
