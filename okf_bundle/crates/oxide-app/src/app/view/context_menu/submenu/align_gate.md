---
okf_version: "0.2"
type: Function
title: align_gate
description: "Altium gating for the Align submenu: pairwise aligns"
resource: crates/oxide-app/src/app/view/context_menu/submenu.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/context_menu/submenu/align_gate
language: rust
---

# align_gate

Altium gating for the Align submenu: pairwise aligns

## Signature

```rust
pub(super) fn align_gate(selected: usize) -> (bool, bool)
```

## Visibility

- `pub(super)`

## Docstring

Altium gating for the Align submenu: pairwise aligns
(Left/Right/Top/Bottom/H/V Centers) need ≥2 items to make sense;
Distribute needs ≥3 (two endpoints + at least one item to space
between them). Returns `(pairwise_enabled, distribute_enabled)`.
Align To Grid works on a single item so it is always enabled.

## Source
Lines 121–123 in `crates/oxide-app/src/app/view/context_menu/submenu.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [submenu](/crates/oxide-app/src/app/view/context_menu/submenu.md) |
| called_by | [align_entries](/crates/oxide-app/src/app/view/context_menu/submenu/align_entries.md) |
