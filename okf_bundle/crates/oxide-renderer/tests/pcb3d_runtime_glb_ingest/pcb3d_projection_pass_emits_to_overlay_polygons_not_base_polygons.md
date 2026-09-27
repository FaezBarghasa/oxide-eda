---
okf_version: "0.2"
type: Function
title: pcb3d_projection_pass_emits_to_overlay_polygons_not_base_polygons
description: "[test]"
resource: crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest/pcb3d_projection_pass_emits_to_overlay_polygons_not_base_polygons
language: rust
---

# pcb3d_projection_pass_emits_to_overlay_polygons_not_base_polygons

[test]

## Signature

```rust
fn pcb3d_projection_pass_emits_to_overlay_polygons_not_base_polygons()
```

## Decorators

- `test`

## Docstring

[test]

## Source
Lines 328–355 in `crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pcb3d_runtime_glb_ingest](/crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest.md) |
| calls | [ingest_runtime_glb](/crates/oxide-renderer/src/pcb3d/glb/ingest_runtime_glb.md) |
| calls | [request](/crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest/request.md) |
| calls | [make_glb_with_json](/crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest/make_glb_with_json.md) |
| calls | [emit_opaque_pass_preview](/crates/oxide-renderer/src/pcb3d/glb/emit_opaque_pass_preview.md) |
| calls | [emit_projection_pass](/crates/oxide-renderer/src/pcb3d/projection/emit_projection_pass.md) |
