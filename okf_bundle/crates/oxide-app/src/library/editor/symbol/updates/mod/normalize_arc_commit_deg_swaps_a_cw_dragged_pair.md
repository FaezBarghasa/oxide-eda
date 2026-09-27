---
okf_version: "0.2"
type: Function
title: normalize_arc_commit_deg_swaps_a_cw_dragged_pair
description: "`normalize_arc_commit_deg` swaps a CW-dragged pair (`end <"
resource: crates/oxide-app/src/library/editor/symbol/updates/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/editor/symbol/updates/mod/normalize_arc_commit_deg_swaps_a_cw_dragged_pair
language: rust
---

# normalize_arc_commit_deg_swaps_a_cw_dragged_pair

`normalize_arc_commit_deg` swaps a CW-dragged pair (`end <

## Signature

```rust
fn normalize_arc_commit_deg_swaps_a_cw_dragged_pair()
```

## Decorators

- `test`

## Docstring

`normalize_arc_commit_deg` swaps a CW-dragged pair (`end <
start`) so the stored endpoints represent the same short arc
under the CCW-wraparound convention, instead of preserving the
(wrong) long-way-around sweep a per-field `rem_euclid` would.
[test]

## Source
Lines 831–835 in `crates/oxide-app/src/library/editor/symbol/updates/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [updates](/crates/oxide-app/src/library/editor/symbol/updates/mod.md) |
