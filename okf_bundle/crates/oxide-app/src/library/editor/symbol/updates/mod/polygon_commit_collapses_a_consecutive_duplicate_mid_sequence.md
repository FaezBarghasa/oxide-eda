---
okf_version: "0.2"
type: Function
title: polygon_commit_collapses_a_consecutive_duplicate_mid_sequence
description: Two slow clicks landing on the same snapped point mid-sequence
resource: crates/oxide-app/src/library/editor/symbol/updates/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/mod/polygon_commit_collapses_a_consecutive_duplicate_mid_sequence
language: rust
---

# polygon_commit_collapses_a_consecutive_duplicate_mid_sequence

Two slow clicks landing on the same snapped point mid-sequence

## Signature

```rust
fn polygon_commit_collapses_a_consecutive_duplicate_mid_sequence()
```

## Decorators

- `test`

## Docstring

Two slow clicks landing on the same snapped point mid-sequence
collapse into one vertex — `[P, P, Q, R]` -> `[P, Q, R]` —
mirroring `oxide_library`'s chain `finalize_ring` dedup pass.
[test]

## Source
Lines 787–800 in `crates/oxide-app/src/library/editor/symbol/updates/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [updates](/crates/oxide-app/src/library/editor/symbol/updates/mod.md) |
| calls | [new_editor](/crates/oxide-app/src/library/editor/symbol/updates/mod/new_editor.md) |
| calls | [apply_symbol_primitive_edit](/crates/oxide-app/src/library/editor/symbol/updates/mod/apply_symbol_primitive_edit.md) |
