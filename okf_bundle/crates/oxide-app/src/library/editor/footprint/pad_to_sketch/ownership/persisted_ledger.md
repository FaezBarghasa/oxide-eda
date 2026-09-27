---
okf_version: "0.2"
type: Function
title: persisted_ledger
description: "The durable ledger carried by `centre`'s `PadAttr`. Empty when the"
resource: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/ownership.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/pad_to_sketch/ownership/persisted_ledger
language: rust
---

# persisted_ledger

The durable ledger carried by `centre`'s `PadAttr`. Empty when the

## Signature

```rust
fn persisted_ledger(sketch: &SketchData, centre: SketchEntityId) -> Vec<SketchEntityId>
```

## Docstring

The durable ledger carried by `centre`'s `PadAttr`. Empty when the
entity is gone, carries no `PadAttr`, or predates the field.

## Source
Lines 103–111 in `crates/oxide-app/src/library/editor/footprint/pad_to_sketch/ownership.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ownership](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/ownership.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
| called_by | [owned_sketch_entities](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/ownership/owned_sketch_entities.md) |
