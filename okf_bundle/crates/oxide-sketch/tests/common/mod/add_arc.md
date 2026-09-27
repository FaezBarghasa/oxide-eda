---
okf_version: "0.2"
type: Function
title: add_arc
resource: crates/oxide-sketch/tests/common/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-sketch/tests/common/mod/add_arc
language: rust
---

# add_arc

## Signature

```rust
impl Sketch { pub fn add_arc(
        &mut self,
        center: SketchEntityId,
        start: SketchEntityId,
        end: SketchEntityId,
        sweep_ccw: bool,
    ) -> SketchEntityId }
```

## Visibility

- `pub`

## Source
Lines 57–76 in `crates/oxide-sketch/tests/common/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [common](/crates/oxide-sketch/tests/common/mod.md) |
