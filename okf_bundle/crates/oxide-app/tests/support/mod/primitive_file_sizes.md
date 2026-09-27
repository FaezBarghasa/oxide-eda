---
okf_version: "0.2"
type: Function
title: primitive_file_sizes
description: "Per-file byte weights for the generated primitives in `library_dir`."
resource: crates/oxide-app/tests/support/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/support/mod/primitive_file_sizes
language: rust
---

# primitive_file_sizes

Per-file byte weights for the generated primitives in `library_dir`.

## Signature

```rust
pub fn primitive_file_sizes(library_dir: &Path) -> Sizes
```

## Visibility

- `pub`

## Docstring

Per-file byte weights for the generated primitives in `library_dir`.

## Source
Lines 550–570 in `crates/oxide-app/tests/support/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [support](/crates/oxide-app/tests/support/mod.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| calls | [metadata](/crates/oxide-output/src/substitution/metadata.md) |
| calls | [first_file_len](/crates/oxide-app/tests/support/mod/first_file_len.md) |
| calls | [dir_total](/crates/oxide-app/tests/support/mod/dir_total.md) |
| called_by | [measure_library_open](/crates/oxide-app/tests/measure_library_open/measure_library_open.md) |
