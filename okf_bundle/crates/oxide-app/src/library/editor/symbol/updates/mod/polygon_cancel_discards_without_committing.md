---
okf_version: "0.2"
type: Function
title: polygon_cancel_discards_without_committing
description: "`PolygonCancel` discards the stash with no commit, regardless"
resource: crates/oxide-app/src/library/editor/symbol/updates/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/mod/polygon_cancel_discards_without_committing
language: rust
---

# polygon_cancel_discards_without_committing

`PolygonCancel` discards the stash with no commit, regardless

## Signature

```rust
fn polygon_cancel_discards_without_committing()
```

## Decorators

- `test`

## Docstring

`PolygonCancel` discards the stash with no commit, regardless
of vertex count.
[test]

## Source
Lines 805–824 in `crates/oxide-app/src/library/editor/symbol/updates/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [updates](/crates/oxide-app/src/library/editor/symbol/updates/mod.md) |
| calls | [new_editor](/crates/oxide-app/src/library/editor/symbol/updates/mod/new_editor.md) |
| calls | [apply_symbol_primitive_edit](/crates/oxide-app/src/library/editor/symbol/updates/mod/apply_symbol_primitive_edit.md) |
