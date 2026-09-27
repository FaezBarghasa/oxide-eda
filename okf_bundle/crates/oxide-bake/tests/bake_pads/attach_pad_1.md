---
okf_version: "0.2"
type: Function
title: attach_pad
description: "Mutate the most-recently-added entity (or the entity with `id`)"
resource: crates/oxide-bake/tests/bake_pads.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-bake/tests/bake_pads/attach_pad_1
language: rust
---

# attach_pad

Mutate the most-recently-added entity (or the entity with `id`)

## Signature

```rust
fn attach_pad(&mut self, id: SketchEntityId, pad: PadAttr)
```

## Docstring

Mutate the most-recently-added entity (or the entity with `id`)
to attach the given `PadAttr`.

## Source
Lines 65–73 in `crates/oxide-bake/tests/bake_pads.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bake_pads](/crates/oxide-bake/tests/bake_pads.md) |
