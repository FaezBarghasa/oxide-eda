---
okf_version: "0.2"
type: Function
title: sketch_placement_char_routes_to_footprint_uniformly
description: Every other Footprint variant already passed through uniformly;
resource: crates/oxide-app/src/library/editor/standalone/footprint.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/standalone/footprint/sketch_placement_char_routes_to_footprint_uniformly
language: rust
---

# sketch_placement_char_routes_to_footprint_uniformly

Every other Footprint variant already passed through uniformly;

## Signature

```rust
fn sketch_placement_char_routes_to_footprint_uniformly()
```

## Decorators

- `test`

## Docstring

Every other Footprint variant already passed through uniformly;
pin that down too so a future special-case regresses loudly.
[test]

## Source
Lines 784–797 in `crates/oxide-app/src/library/editor/standalone/footprint.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint](/crates/oxide-app/src/library/editor/standalone/footprint.md) |
| calls | [translate_footprint_canvas_msg](/crates/oxide-app/src/library/editor/standalone/footprint/translate_footprint_canvas_msg.md) |
| calls | [wrap](/crates/oxide-app/src/library/editor/standalone/footprint/wrap.md) |
