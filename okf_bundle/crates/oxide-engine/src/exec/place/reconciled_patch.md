---
okf_version: "0.2"
type: Function
title: reconciled_patch
description: Reconcile junction dots after a command mutated wire geometry (move /
resource: crates/oxide-engine/src/exec/place.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-engine/src/exec/place/reconciled_patch
language: rust
---

# reconciled_patch

Reconcile junction dots after a command mutated wire geometry (move /

## Signature

```rust
impl Engine { fn reconciled_patch(&mut self, items: &[SelectedItem]) -> DocumentPatch }
```

## Docstring

Reconcile junction dots after a command mutated wire geometry (move /
rotate / mirror), removed wires (delete), or added one (place), and
return the document patch including `JUNCTIONS` if any dot was
minted or removed. Shared by all five arms so none of them can drift
back into leaving a junction-less T (issue #402) or a stale,
silently net-merging dot (issue #422) behind.

## Source
Lines 13–19 in `crates/oxide-engine/src/exec/place.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [place](/crates/oxide-engine/src/exec/place.md) |
