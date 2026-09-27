---
okf_version: "0.2"
type: Function
title: polygon_commit_drops_duplicate_closing_vertex
description: A trailing vertex equal to the first (a plain click landed
resource: crates/oxide-app/src/library/editor/symbol/updates/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/mod/polygon_commit_drops_duplicate_closing_vertex
language: rust
---

# polygon_commit_drops_duplicate_closing_vertex

A trailing vertex equal to the first (a plain click landed

## Signature

```rust
fn polygon_commit_drops_duplicate_closing_vertex()
```

## Decorators

- `test`

## Docstring

A trailing vertex equal to the first (a plain click landed
exactly on vertex 0's snapped grid position without triggering
the tolerance-based close gesture) is dropped before
committing, so the ring doesn't double its closing edge.
[test]

## Source
Lines 735–762 in `crates/oxide-app/src/library/editor/symbol/updates/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [updates](/crates/oxide-app/src/library/editor/symbol/updates/mod.md) |
| calls | [new_editor](/crates/oxide-app/src/library/editor/symbol/updates/mod/new_editor.md) |
| calls | [apply_symbol_primitive_edit](/crates/oxide-app/src/library/editor/symbol/updates/mod/apply_symbol_primitive_edit.md) |
