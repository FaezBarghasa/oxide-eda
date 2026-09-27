---
okf_version: "0.2"
type: Function
title: fixture_footprint_with_bga_array
description: "Build a footprint editor with one Linear array + BgaRowCol numbering,"
resource: crates/oxide-app/tests/regression/library_bga.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/regression/library_bga/fixture_footprint_with_bga_array
language: rust
---

# fixture_footprint_with_bga_array

Build a footprint editor with one Linear array + BgaRowCol numbering,

## Signature

```rust
fn fixture_footprint_with_bga_array(stem: &str) -> (Oxide, oxide_sketch::array::ArrayId)
```

## Docstring

Build a footprint editor with one Linear array + BgaRowCol numbering,
plant it as the active tab, and return the array's id so the test
can target it by id (the dispatcher matches arrays by id).

## Source
Lines 21–76 in `crates/oxide-app/tests/regression/library_bga.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_bga](/crates/oxide-app/tests/regression/library_bga.md) |
| called_by | [v025_bga_set_skip_letters_round_trips_bool](/crates/oxide-app/tests/regression/library_bga/v025_bga_set_skip_letters_round_trips_bool.md) |
| called_by | [v025_bga_set_start_col_parses_valid_integer](/crates/oxide-app/tests/regression/library_bga/v025_bga_set_start_col_parses_valid_integer.md) |
| called_by | [v025_bga_set_start_col_rejects_non_numeric_input](/crates/oxide-app/tests/regression/library_bga/v025_bga_set_start_col_rejects_non_numeric_input.md) |
| called_by | [v025_bga_set_start_row_rejects_non_alphabetic_input](/crates/oxide-app/tests/regression/library_bga/v025_bga_set_start_row_rejects_non_alphabetic_input.md) |
| called_by | [v025_bga_set_start_row_uppercases_lowercase_input](/crates/oxide-app/tests/regression/library_bga/v025_bga_set_start_row_uppercases_lowercase_input.md) |
