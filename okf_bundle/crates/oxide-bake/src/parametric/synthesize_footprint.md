---
okf_version: "0.2"
type: Function
title: synthesize_footprint
resource: crates/oxide-bake/src/parametric.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-bake"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-27T13:06:33Z"
concept_id: crates/oxide-bake/src/parametric/synthesize_footprint
language: rust
---

# synthesize_footprint

## Signature

```rust
impl Ipc7351Synthesizer { fn synthesize_footprint(
        &self,
        dimensions: &PackageDimensions,
        density: DensityLevel,
    ) -> Result<Footprint, BakeError> }
```

## Source
Lines 36–46 in `crates/oxide-bake/src/parametric.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [parametric](/crates/oxide-bake/src/parametric.md) |
