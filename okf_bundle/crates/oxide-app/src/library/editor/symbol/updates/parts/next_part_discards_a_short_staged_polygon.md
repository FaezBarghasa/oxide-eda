---
okf_version: "0.2"
type: Function
title: next_part_discards_a_short_staged_polygon
description: A short (< 3 vertex) stash discards silently on part switch —
resource: crates/oxide-app/src/library/editor/symbol/updates/parts.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/parts/next_part_discards_a_short_staged_polygon
language: rust
---

# next_part_discards_a_short_staged_polygon

A short (< 3 vertex) stash discards silently on part switch —

## Signature

```rust
fn next_part_discards_a_short_staged_polygon()
```

## Decorators

- `test`

## Docstring

A short (< 3 vertex) stash discards silently on part switch —
no graphic, no undo entry — same as `SetTool`'s flush.
[test]

## Source
Lines 158–169 in `crates/oxide-app/src/library/editor/symbol/updates/parts.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parts](/crates/oxide-app/src/library/editor/symbol/updates/parts.md) |
| calls | [new_editor](/crates/oxide-app/src/library/editor/symbol/updates/parts/new_editor.md) |
| calls | [apply_symbol_parts](/crates/oxide-app/src/library/editor/symbol/updates/parts/apply_symbol_parts.md) |
