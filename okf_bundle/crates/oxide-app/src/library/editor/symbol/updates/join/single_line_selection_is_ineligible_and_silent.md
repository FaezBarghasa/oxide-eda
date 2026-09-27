---
okf_version: "0.2"
type: Function
title: single_line_selection_is_ineligible_and_silent
description: "A single selected `Line` is ineligible — it can never close on"
resource: crates/oxide-app/src/library/editor/symbol/updates/join.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/join/single_line_selection_is_ineligible_and_silent
language: rust
---

# single_line_selection_is_ineligible_and_silent

A single selected `Line` is ineligible — it can never close on

## Signature

```rust
fn single_line_selection_is_ineligible_and_silent()
```

## Decorators

- `test`

## Docstring

A single selected `Line` is ineligible — it can never close on
its own — and the op is a silent no-op (matching the disabled
menu row), not a misleading "Selection is degenerate" chain
error.
[test]

## Source
Lines 536–549 in `crates/oxide-app/src/library/editor/symbol/updates/join.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [join](/crates/oxide-app/src/library/editor/symbol/updates/join.md) |
| calls | [new_editor](/crates/oxide-app/src/library/editor/symbol/updates/join/new_editor.md) |
| calls | [push_line](/crates/oxide-app/src/library/editor/symbol/updates/join/push_line.md) |
| calls | [Graphic](/crates/oxide-types/src/schematic/mod/Graphic.md) |
| calls | [apply_symbol_join](/crates/oxide-app/src/library/editor/symbol/updates/join/apply_symbol_join.md) |
