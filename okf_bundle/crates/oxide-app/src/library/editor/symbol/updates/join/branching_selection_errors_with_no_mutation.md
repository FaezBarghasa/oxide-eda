---
okf_version: "0.2"
type: Function
title: branching_selection_errors_with_no_mutation
description: "A branching (T-junction) selection errors, mutates nothing, and"
resource: crates/oxide-app/src/library/editor/symbol/updates/join.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/join/branching_selection_errors_with_no_mutation
language: rust
---

# branching_selection_errors_with_no_mutation

A branching (T-junction) selection errors, mutates nothing, and

## Signature

```rust
fn branching_selection_errors_with_no_mutation()
```

## Decorators

- `test`

## Docstring

A branching (T-junction) selection errors, mutates nothing, and
pushes no undo entry.
[test]

## Source
Lines 350–378 in `crates/oxide-app/src/library/editor/symbol/updates/join.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [join](/crates/oxide-app/src/library/editor/symbol/updates/join.md) |
| calls | [new_editor](/crates/oxide-app/src/library/editor/symbol/updates/join/new_editor.md) |
| calls | [push_line](/crates/oxide-app/src/library/editor/symbol/updates/join/push_line.md) |
| calls | [apply_symbol_join](/crates/oxide-app/src/library/editor/symbol/updates/join/apply_symbol_join.md) |
