---
okf_version: "0.2"
type: Function
title: mint_body3d_extrudes_courtyard
description: 3D Body mint populates body_3d as an Extrude body whose outline is the
resource: crates/oxide-app/src/library/editor/footprint/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/tests/mint_body3d_extrudes_courtyard
language: rust
---

# mint_body3d_extrudes_courtyard

3D Body mint populates body_3d as an Extrude body whose outline is the

## Signature

```rust
fn mint_body3d_extrudes_courtyard()
```

## Decorators

- `test`

## Docstring

3D Body mint populates body_3d as an Extrude body whose outline is the
courtyard, so the CPU preview shows a solid immediately.
[test]

## Source
Lines 88–107 in `crates/oxide-app/src/library/editor/footprint/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-app/src/library/editor/footprint/tests.md) |
| calls | [mint_box_from_courtyard](/crates/oxide-app/src/library/editor/footprint/body3d_mint/mint_box_from_courtyard.md) |
