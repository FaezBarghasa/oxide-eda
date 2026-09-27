---
okf_version: "0.2"
type: Function
title: segments_for
description: "Build the `ChainSegment`s + max source stroke width for `indices`."
resource: crates/oxide-app/src/library/editor/symbol/updates/join.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/join/segments_for
language: rust
---

# segments_for

Build the `ChainSegment`s + max source stroke width for `indices`.

## Signature

```rust
fn segments_for(sym: &Symbol, indices: &[usize]) -> (Vec<ChainSegment>, f64)
```

## Docstring

Build the `ChainSegment`s + max source stroke width for `indices`.
Assumes `state::selection_is_join_eligible` already confirmed every
index names a valid Line/Arc graphic — still defensive (`.get`, not
indexing) since a stale index should never reach here but must not
panic if it somehow does.

## Source
Lines 140–166 in `crates/oxide-app/src/library/editor/symbol/updates/join.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [join](/crates/oxide-app/src/library/editor/symbol/updates/join.md) |
| called_by | [apply_symbol_join](/crates/oxide-app/src/library/editor/symbol/updates/join/apply_symbol_join.md) |
