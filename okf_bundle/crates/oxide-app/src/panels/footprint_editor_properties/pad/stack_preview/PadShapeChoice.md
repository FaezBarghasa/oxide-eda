---
okf_version: "0.2"
type: Class
title: PadShapeChoice
description: "v0.20 — pick_list-friendly proxy for `oxide_library::PadShape`."
resource: crates/oxide-app/src/panels/footprint_editor_properties/pad/stack_preview.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/footprint_editor_properties/pad/stack_preview/PadShapeChoice
language: rust
---

# PadShapeChoice

v0.20 — pick_list-friendly proxy for `oxide_library::PadShape`.

## Signature

```rust
pub(super) enum PadShapeChoice
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq)`

## Visibility

- `pub(super)`

## Docstring

v0.20 — pick_list-friendly proxy for `oxide_library::PadShape`.
Mirrors Altium's COPPER → Shape dropdown verbatim minus
"Custom Shape" (sketch mode owns freeform geometry):
Round / Rectangular / Octagonal / Rounded Rectangle /
Chamfered Rectangle / Donut.
Schema-mapping notes:
- Octagonal / Donut have no native variant on
`oxide_library::PadShape` yet; both fall back to Round at
bake. Round trip preserves the picker selection across
sessions once we add schema variants in v0.21.
- Chamfered Rectangle uses the existing `Chamfered` variant
with sensible defaults (25% chamfer, all corners).
[derive(Debug, Clone, Copy, PartialEq, Eq)]

## Source
Lines 514–521 in `crates/oxide-app/src/panels/footprint_editor_properties/pad/stack_preview.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [stack_preview](/crates/oxide-app/src/panels/footprint_editor_properties/pad/stack_preview.md) |
