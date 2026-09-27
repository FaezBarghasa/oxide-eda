---
okf_version: "0.2"
type: Function
title: selection_with_rectangle_is_a_no_op
description: A selection containing a non-Line/Arc graphic (Rectangle) is a
resource: crates/oxide-app/src/library/editor/symbol/updates/join.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/join/selection_with_rectangle_is_a_no_op
language: rust
---

# selection_with_rectangle_is_a_no_op

A selection containing a non-Line/Arc graphic (Rectangle) is a

## Signature

```rust
fn selection_with_rectangle_is_a_no_op()
```

## Decorators

- `test`

## Docstring

A selection containing a non-Line/Arc graphic (Rectangle) is a
no-op: nothing removed, nothing appended, no undo entry, no
status message (dispatch-level guard, distinct from a chain
error).
[test]

## Source
Lines 385–407 in `crates/oxide-app/src/library/editor/symbol/updates/join.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [join](/crates/oxide-app/src/library/editor/symbol/updates/join.md) |
| calls | [new_editor](/crates/oxide-app/src/library/editor/symbol/updates/join/new_editor.md) |
| calls | [push_line](/crates/oxide-app/src/library/editor/symbol/updates/join/push_line.md) |
| calls | [apply_symbol_join](/crates/oxide-app/src/library/editor/symbol/updates/join/apply_symbol_join.md) |
