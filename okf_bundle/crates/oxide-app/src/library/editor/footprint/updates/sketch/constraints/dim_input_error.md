---
okf_version: "0.2"
type: Function
title: dim_input_error
description: "The `dimension_input` text when it is present but unreadable as a"
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/constraints.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/constraints/dim_input_error
language: rust
---

# dim_input_error

The `dimension_input` text when it is present but unreadable as a

## Signature

```rust
fn dim_input_error(dimension_input: &str) -> Option<String>
```

## Docstring

The `dimension_input` text when it is present but unreadable as a
number, `None` when it is empty or valid.

An empty field is not an error on its own: most constraint tags
(Coincident, Parallel, Horizontal, …) take no dimension at all, so
the failure there is a selection mismatch, not a bad number.

## Source
Lines 256–262 in `crates/oxide-app/src/library/editor/footprint/updates/sketch/constraints.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [constraints](/crates/oxide-app/src/library/editor/footprint/updates/sketch/constraints.md) |
| called_by | [add_constraint_for_selection](/crates/oxide-app/src/library/editor/footprint/updates/sketch/constraints/add_constraint_for_selection.md) |
