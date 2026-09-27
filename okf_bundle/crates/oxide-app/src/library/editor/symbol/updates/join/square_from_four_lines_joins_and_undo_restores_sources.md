---
okf_version: "0.2"
type: Function
title: square_from_four_lines_joins_and_undo_restores_sources
description: Square from 4 selected lines joins into 1 polygon; the 4
resource: crates/oxide-app/src/library/editor/symbol/updates/join.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/join/square_from_four_lines_joins_and_undo_restores_sources
language: rust
---

# square_from_four_lines_joins_and_undo_restores_sources

Square from 4 selected lines joins into 1 polygon; the 4

## Signature

```rust
fn square_from_four_lines_joins_and_undo_restores_sources()
```

## Decorators

- `test`

## Docstring

Square from 4 selected lines joins into 1 polygon; the 4
sources are gone; undo restores all 4 sources and removes the
polygon.
[test]

## Source
Lines 232–268 in `crates/oxide-app/src/library/editor/symbol/updates/join.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [join](/crates/oxide-app/src/library/editor/symbol/updates/join.md) |
| calls | [square_editor](/crates/oxide-app/src/library/editor/symbol/updates/join/square_editor.md) |
| calls | [apply_symbol_join](/crates/oxide-app/src/library/editor/symbol/updates/join/apply_symbol_join.md) |
