---
okf_version: "0.2"
type: Function
title: drawing_uuid
description: "`SchDrawing` doesn't implement `HasUuid` (its uuid lives inside each"
resource: crates/oxide-engine/src/transform/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-engine/src/transform/mod/drawing_uuid
language: rust
---

# drawing_uuid

`SchDrawing` doesn't implement `HasUuid` (its uuid lives inside each

## Signature

```rust
fn drawing_uuid(d: &SchDrawing) -> uuid::Uuid
```

## Docstring

`SchDrawing` doesn't implement `HasUuid` (its uuid lives inside each
enum variant, not on a common struct field) — shared by
`contains_selected_item`, `remove_selected_item` and
`move_selected_item` so the five-variant match lives in one place.

## Source
Lines 517–525 in `crates/oxide-engine/src/transform/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [transform](/crates/oxide-engine/src/transform/mod.md) |
| called_by | [contains_selected_item](/crates/oxide-engine/src/transform/mod/contains_selected_item.md) |
| called_by | [move_selected_item](/crates/oxide-engine/src/transform/mod/move_selected_item.md) |
| called_by | [remove_selected_item](/crates/oxide-engine/src/transform/mod/remove_selected_item.md) |
