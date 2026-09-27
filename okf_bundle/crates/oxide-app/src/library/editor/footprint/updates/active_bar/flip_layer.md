---
okf_version: "0.2"
type: Function
title: flip_layer
description: Swap a layer between the front and back side. Anything without an
resource: crates/oxide-app/src/library/editor/footprint/updates/active_bar.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/active_bar/flip_layer
language: rust
---

# flip_layer

Swap a layer between the front and back side. Anything without an

## Signature

```rust
fn flip_layer(layer: &oxide_library::LayerId) -> oxide_library::LayerId
```

## Docstring

Swap a layer between the front and back side. Anything without an
`F.` / `B.` prefix (`*.Cu`, bare names) is side-agnostic and passes
through unchanged.

## Source
Lines 488–498 in `crates/oxide-app/src/library/editor/footprint/updates/active_bar.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [active_bar](/crates/oxide-app/src/library/editor/footprint/updates/active_bar.md) |
