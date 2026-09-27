---
okf_version: "0.2"
type: Function
title: read_bga_config
description: Read back the current BgaRowCol triple from the active footprint
resource: crates/oxide-app/tests/regression/library_bga.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/regression/library_bga/read_bga_config
language: rust
---

# read_bga_config

Read back the current BgaRowCol triple from the active footprint

## Signature

```rust
fn read_bga_config(app: &Oxide) -> (bool, char, u32)
```

## Docstring

Read back the current BgaRowCol triple from the active footprint
editor's first array. Panics if the array isn't BgaRowCol — that
would indicate the test setup got clobbered.

## Source
Lines 81–109 in `crates/oxide-app/tests/regression/library_bga.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_bga](/crates/oxide-app/tests/regression/library_bga.md) |
| called_by | [v025_bga_set_start_col_rejects_non_numeric_input](/crates/oxide-app/tests/regression/library_bga/v025_bga_set_start_col_rejects_non_numeric_input.md) |
| called_by | [v025_bga_set_start_row_rejects_non_alphabetic_input](/crates/oxide-app/tests/regression/library_bga/v025_bga_set_start_row_rejects_non_alphabetic_input.md) |
