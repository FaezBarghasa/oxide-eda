---
okf_version: "0.2"
type: Function
title: polygon_commit_with_fewer_than_three_vertices_is_discarded
description: "Fewer than 3 collected vertices — `PolygonCommit` is a silent"
resource: crates/oxide-app/src/library/editor/symbol/updates/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/mod/polygon_commit_with_fewer_than_three_vertices_is_discarded
language: rust
---

# polygon_commit_with_fewer_than_three_vertices_is_discarded

Fewer than 3 collected vertices — `PolygonCommit` is a silent

## Signature

```rust
fn polygon_commit_with_fewer_than_three_vertices_is_discarded()
```

## Decorators

- `test`

## Docstring

Fewer than 3 collected vertices — `PolygonCommit` is a silent
discard: no graphic, no undo snapshot, stash still clears.
[test]

## Source
Lines 690–705 in `crates/oxide-app/src/library/editor/symbol/updates/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [updates](/crates/oxide-app/src/library/editor/symbol/updates/mod.md) |
| calls | [new_editor](/crates/oxide-app/src/library/editor/symbol/updates/mod/new_editor.md) |
| calls | [apply_symbol_primitive_edit](/crates/oxide-app/src/library/editor/symbol/updates/mod/apply_symbol_primitive_edit.md) |
