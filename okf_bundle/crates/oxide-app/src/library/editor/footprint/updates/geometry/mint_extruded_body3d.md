---
okf_version: "0.2"
type: Function
title: mint_extruded_body3d
resource: crates/oxide-app/src/library/editor/footprint/updates/geometry.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/geometry/mint_extruded_body3d
language: rust
---

# mint_extruded_body3d

## Signature

```rust
fn mint_extruded_body3d(editor: &mut crate::app::FootprintEditorState)
```

## Source
Lines 309–317 in `crates/oxide-app/src/library/editor/footprint/updates/geometry.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [geometry](/crates/oxide-app/src/library/editor/footprint/updates/geometry.md) |
| calls | [mint_extruded_from_fab](/crates/oxide-app/src/library/editor/footprint/body3d_mint/mint_extruded_from_fab.md) |
| called_by | [apply](/crates/oxide-app/src/library/editor/footprint/updates/geometry/apply.md) |
