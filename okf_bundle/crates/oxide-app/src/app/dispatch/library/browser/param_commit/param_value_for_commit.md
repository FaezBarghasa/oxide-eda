---
okf_version: "0.2"
type: Function
title: param_value_for_commit
description: Decide the value a parameter cell commits.
resource: crates/oxide-app/src/app/dispatch/library/browser/param_commit.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/dispatch/library/browser/param_commit/param_value_for_commit
language: rust
---

# param_value_for_commit

Decide the value a parameter cell commits.

## Signature

```rust
pub(super) fn param_value_for_commit(
    existing: Option<&ParamValue>,
    buf: &str,
) -> Result<ParamValue, ParamRefusal>
```

## Visibility

- `pub(super)`

## Docstring

Decide the value a parameter cell commits.

`existing` is the value currently stored under the cell's key
(`None` for a key the row does not have yet). Returns `Err` when the
cell is typed and `buf` cannot be read as that type — the caller must
then leave the stored value untouched.

## Source
Lines 38–81 in `crates/oxide-app/src/app/dispatch/library/browser/param_commit.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [param_commit](/crates/oxide-app/src/app/dispatch/library/browser/param_commit.md) |
| calls | [refusal](/crates/oxide-app/src/app/dispatch/library/browser/param_commit/refusal.md) |
| called_by | [handle_browser_cell_commit](/crates/oxide-app/src/app/dispatch/library/browser/mod/handle_browser_cell_commit.md) |
| called_by | [a_bool_cell_refuses_a_buffer_it_cannot_read_rather_than_retyping](/crates/oxide-app/src/app/dispatch/library/browser/param_commit/a_bool_cell_refuses_a_buffer_it_cannot_read_rather_than_retyping.md) |
| called_by | [a_measurement_cell_keeps_its_unit_when_the_buffer_parses](/crates/oxide-app/src/app/dispatch/library/browser/param_commit/a_measurement_cell_keeps_its_unit_when_the_buffer_parses.md) |
| called_by | [a_measurement_cell_refuses_a_buffer_that_is_not_a_number](/crates/oxide-app/src/app/dispatch/library/browser/param_commit/a_measurement_cell_refuses_a_buffer_that_is_not_a_number.md) |
| called_by | [a_number_cell_commits_a_parseable_buffer](/crates/oxide-app/src/app/dispatch/library/browser/param_commit/a_number_cell_commits_a_parseable_buffer.md) |
| called_by | [a_number_cell_refuses_a_buffer_that_is_not_a_number](/crates/oxide-app/src/app/dispatch/library/browser/param_commit/a_number_cell_refuses_a_buffer_that_is_not_a_number.md) |
| called_by | [an_untyped_cell_takes_the_buffer_verbatim](/crates/oxide-app/src/app/dispatch/library/browser/param_commit/an_untyped_cell_takes_the_buffer_verbatim.md) |
| called_by | [refusing_a_boolean_buffer_does_not_touch_untyped_cells](/crates/oxide-app/src/app/dispatch/library/browser/param_commit/refusing_a_boolean_buffer_does_not_touch_untyped_cells.md) |
