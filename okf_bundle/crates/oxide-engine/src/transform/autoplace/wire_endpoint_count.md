---
okf_version: "0.2"
type: Function
title: wire_endpoint_count
description: "Number of the sheet's wires that terminate (start or end) at `point`."
resource: crates/oxide-engine/src/transform/autoplace.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-engine/src/transform/autoplace/wire_endpoint_count
language: rust
---

# wire_endpoint_count

Number of the sheet's wires that terminate (start or end) at `point`.

## Signature

```rust
fn wire_endpoint_count(
    point: oxide_types::schematic::Point,
    document: &SchematicSheet,
    tolerance: f64,
) -> usize
```

## Docstring

Number of the sheet's wires that terminate (start or end) at `point`.

## Source
Lines 431–447 in `crates/oxide-engine/src/transform/autoplace.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [autoplace](/crates/oxide-engine/src/transform/autoplace.md) |
| called_by | [wire_meeting_justifies_junction](/crates/oxide-engine/src/transform/autoplace/wire_meeting_justifies_junction.md) |
