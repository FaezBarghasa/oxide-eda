---
okf_version: "0.2"
type: Function
title: reorder_slot
description: "Helper: given a Vec and a reference uuid, return the"
resource: crates/oxide-engine/src/exec/structure.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-engine/src/exec/structure/reorder_slot
language: rust
---

# reorder_slot

Helper: given a Vec and a reference uuid, return the

## Signature

```rust
impl Engine { fn reorder_slot(
                    vec: &[T],
                    uuid_of: impl Fn(&T) -> uuid::Uuid,
                    direction: ReorderDirection,
                ) -> Option<usize> }
```

## Type Parameters

- `T`

## Docstring

Helper: given a Vec and a reference uuid, return the
insert position for JustAbove (ref_idx + 1) or
JustBelow (ref_idx). Returns None when the reference
uuid isn't in the Vec — caller falls back to
no-change.

## Source
Lines 249–264 in `crates/oxide-engine/src/exec/structure.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [structure](/crates/oxide-engine/src/exec/structure.md) |
