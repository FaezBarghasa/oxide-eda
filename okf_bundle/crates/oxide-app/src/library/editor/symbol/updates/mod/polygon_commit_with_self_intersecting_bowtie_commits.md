---
okf_version: "0.2"
type: Function
title: polygon_commit_with_self_intersecting_bowtie_commits
description: A self-intersecting bowtie whose crossed lobes cancel to
resource: crates/oxide-app/src/library/editor/symbol/updates/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/mod/polygon_commit_with_self_intersecting_bowtie_commits
language: rust
---

# polygon_commit_with_self_intersecting_bowtie_commits

A self-intersecting bowtie whose crossed lobes cancel to

## Signature

```rust
fn polygon_commit_with_self_intersecting_bowtie_commits()
```

## Decorators

- `test`

## Docstring

A self-intersecting bowtie whose crossed lobes cancel to
exactly zero NET shoelace area still commits — it has real 2D
extent and renders even-odd, unlike a genuinely collinear
(zero-width) ring.
[test]

## Source
Lines 769–781 in `crates/oxide-app/src/library/editor/symbol/updates/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [updates](/crates/oxide-app/src/library/editor/symbol/updates/mod.md) |
| calls | [new_editor](/crates/oxide-app/src/library/editor/symbol/updates/mod/new_editor.md) |
| calls | [apply_symbol_primitive_edit](/crates/oxide-app/src/library/editor/symbol/updates/mod/apply_symbol_primitive_edit.md) |
