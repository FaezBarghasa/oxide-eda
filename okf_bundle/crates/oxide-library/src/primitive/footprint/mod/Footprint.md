---
okf_version: "0.2"
type: Class
title: Footprint
description: "Reusable PCB primitive. Bound by `Component::footprint_ref`."
resource: crates/oxide-library/src/primitive/footprint/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/primitive/footprint/mod/Footprint
language: rust
---

# Footprint

Reusable PCB primitive. Bound by `Component::footprint_ref`.

## Signature

```rust
pub struct Footprint
```

## Decorators

- `derive(Clone, Debug, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Reusable PCB primitive. Bound by `Component::footprint_ref`.
[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]

## Methods

- `uuid`
- `name`
- `anchor`
- `pads`
- `courtyard`
- `silk_f`
- `silk_b`
- `fab_f`
- `fab_b`
- `body_3d`
- `step_attachment`
- `pcb_params`
- `version`
- `released`
- `created`
- `updated`
- `schema_version`
- `sketch`
- `pours`
- `keepouts`
- `cutouts`
- `v_scores`
- `mask_openings`
- `mask_excludes`
- `paste_apertures`
- `description`
- `default_designator`
- `component_type`
- `height_mm`

## Source
Lines 305–395 in `crates/oxide-library/src/primitive/footprint/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint](/crates/oxide-library/src/primitive/footprint/mod.md) |
