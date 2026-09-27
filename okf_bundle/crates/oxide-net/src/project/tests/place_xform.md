---
okf_version: "0.2"
type: Function
title: place_xform
resource: crates/oxide-net/src/project/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-net/src/project/tests/place_xform
language: rust
---

# place_xform

## Signature

```rust
fn place_xform(
    sheet: &mut SchematicSheet,
    reference: &str,
    lib_id: &str,
    origin: Point,
    rotation: f64,
    mirror_x: bool,
    mirror_y: bool,
)
```

## Source
Lines 154–168 in `crates/oxide-net/src/project/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-net/src/project/tests.md) |
| calls | [place](/crates/oxide-net/src/project/tests/place.md) |
| called_by | [equivalence_gate_root_only](/crates/oxide-net/src/project/tests/equivalence_gate_root_only.md) |
