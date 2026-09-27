---
okf_version: "0.2"
type: Function
title: outline_only_rule_area
description: "Outline-only rule/keepout area: fully transparent fill (alpha 0), the"
resource: crates/oxide-gfx/src/scene/scenario_tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-gfx/src/scene/scenario_tests/outline_only_rule_area
language: rust
---

# outline_only_rule_area

Outline-only rule/keepout area: fully transparent fill (alpha 0), the

## Signature

```rust
fn outline_only_rule_area() -> GpuPolygon
```

## Docstring

Outline-only rule/keepout area: fully transparent fill (alpha 0), the
stroke carries all the visible signal.

## Source
Lines 104–111 in `crates/oxide-gfx/src/scene/scenario_tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [scenario_tests](/crates/oxide-gfx/src/scene/scenario_tests.md) |
| called_by | [triangulate_outline_only_rule_area_still_emits_a_stroke](/crates/oxide-gfx/src/scene/scenario_tests/triangulate_outline_only_rule_area_still_emits_a_stroke.md) |
