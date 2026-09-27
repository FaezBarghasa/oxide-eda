---
okf_version: "0.2"
type: Function
title: all_part0_selection_keeps_part_zero_on_the_result
description: A selection whose sources are all shared (part 0) keeps the
resource: crates/oxide-app/src/library/editor/symbol/updates/join.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/join/all_part0_selection_keeps_part_zero_on_the_result
language: rust
---

# all_part0_selection_keeps_part_zero_on_the_result

A selection whose sources are all shared (part 0) keeps the

## Signature

```rust
fn all_part0_selection_keeps_part_zero_on_the_result()
```

## Decorators

- `test`

## Docstring

A selection whose sources are all shared (part 0) keeps the
result on part 0 — it must not get silently rescoped onto the
active unit (default 1), which would remove the shared body
from every other unit (part 0 is admitted by hit-test and
box-select on every unit).
[test]

## Source
Lines 466–474 in `crates/oxide-app/src/library/editor/symbol/updates/join.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [join](/crates/oxide-app/src/library/editor/symbol/updates/join.md) |
| calls | [square_editor](/crates/oxide-app/src/library/editor/symbol/updates/join/square_editor.md) |
| calls | [apply_symbol_join](/crates/oxide-app/src/library/editor/symbol/updates/join/apply_symbol_join.md) |
