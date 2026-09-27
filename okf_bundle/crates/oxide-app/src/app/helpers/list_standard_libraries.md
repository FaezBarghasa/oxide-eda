---
okf_version: "0.2"
type: Function
title: list_standard_libraries
description: List .standard_sym filenames in a directory.
resource: crates/oxide-app/src/app/helpers.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/app/helpers/list_standard_libraries
language: rust
---

# list_standard_libraries

List .standard_sym filenames in a directory.

## Signature

```rust
pub(super) fn list_standard_libraries(dir: &std::path::Path) -> Vec<String>
```

## Visibility

- `pub(super)`

## Docstring

List .standard_sym filenames in a directory.

Read once, from `Oxide::new()`, into the Components panel's library
pick_list — so a directory that cannot be read leaves that list empty
for the whole session. `list_dir_or_report` surfaces that instead of
letting it read as "no standard libraries installed".

## Source
Lines 58–72 in `crates/oxide-app/src/app/helpers.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [helpers](/crates/oxide-app/src/app/helpers.md) |
| calls | [list_dir_or_report](/crates/oxide-app/src/app/dir_listing/list_dir_or_report.md) |
