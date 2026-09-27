---
okf_version: "0.2"
type: Function
title: junction_at
resource: crates/oxide-engine/src/transform/autoplace.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-engine/src/transform/autoplace/junction_at
language: rust
---

# junction_at

## Signature

```rust
fn junction_at(point: oxide_types::schematic::Point) -> oxide_types::schematic::Junction
```

## Source
Lines 362–371 in `crates/oxide-engine/src/transform/autoplace.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [autoplace](/crates/oxide-engine/src/transform/autoplace.md) |
| called_by | [junctions_under_new_wire](/crates/oxide-engine/src/transform/autoplace/junctions_under_new_wire.md) |
| called_by | [needed_junction](/crates/oxide-engine/src/transform/autoplace/needed_junction.md) |
