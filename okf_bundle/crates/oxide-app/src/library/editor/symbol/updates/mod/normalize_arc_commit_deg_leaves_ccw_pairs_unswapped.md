---
okf_version: "0.2"
type: Function
title: normalize_arc_commit_deg_leaves_ccw_pairs_unswapped
description: An already-CCW (non-wrapped) pair is untouched beyond the
resource: crates/oxide-app/src/library/editor/symbol/updates/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/mod/normalize_arc_commit_deg_leaves_ccw_pairs_unswapped
language: rust
---

# normalize_arc_commit_deg_leaves_ccw_pairs_unswapped

An already-CCW (non-wrapped) pair is untouched beyond the

## Signature

```rust
fn normalize_arc_commit_deg_leaves_ccw_pairs_unswapped()
```

## Decorators

- `test`

## Docstring

An already-CCW (non-wrapped) pair is untouched beyond the
canonicalising `rem_euclid` — no swap, matching the "already
correct" arcs this whole pass leaves unchanged.
[test]

## Source
Lines 841–843 in `crates/oxide-app/src/library/editor/symbol/updates/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [updates](/crates/oxide-app/src/library/editor/symbol/updates/mod.md) |
