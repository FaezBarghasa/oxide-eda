---
okf_version: "0.2"
type: Function
title: arc_and_lines_join
description: A triangle built from two lines and one arc side joins fine.
resource: crates/oxide-app/src/library/editor/symbol/updates/join.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/join/arc_and_lines_join
language: rust
---

# arc_and_lines_join

A triangle built from two lines and one arc side joins fine.

## Signature

```rust
fn arc_and_lines_join()
```

## Decorators

- `test`

## Docstring

A triangle built from two lines and one arc side joins fine.
[test]

## Source
Lines 315–345 in `crates/oxide-app/src/library/editor/symbol/updates/join.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [join](/crates/oxide-app/src/library/editor/symbol/updates/join.md) |
| calls | [new_editor](/crates/oxide-app/src/library/editor/symbol/updates/join/new_editor.md) |
| calls | [push_line](/crates/oxide-app/src/library/editor/symbol/updates/join/push_line.md) |
| calls | [apply_symbol_join](/crates/oxide-app/src/library/editor/symbol/updates/join/apply_symbol_join.md) |
