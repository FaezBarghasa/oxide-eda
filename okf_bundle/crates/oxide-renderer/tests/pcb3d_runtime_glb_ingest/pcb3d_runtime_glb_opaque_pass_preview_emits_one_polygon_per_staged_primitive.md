---
okf_version: "0.2"
type: Function
title: pcb3d_runtime_glb_opaque_pass_preview_emits_one_polygon_per_staged_primitive
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
concept_id: crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest/pcb3d_runtime_glb_opaque_pass_preview_emits_one_polygon_per_staged_primitive
language: rust
---

# pcb3d_runtime_glb_opaque_pass_preview_emits_one_polygon_per_staged_primitive

[test]

## Signature

```rust
fn pcb3d_runtime_glb_opaque_pass_preview_emits_one_polygon_per_staged_primitive()
```

## Decorators

- `test`

## Docstring

[test]

## Source
Lines 230–258 in `crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pcb3d_runtime_glb_ingest](/crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest.md) |
| calls | [make_glb_with_json](/crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest/make_glb_with_json.md) |
| calls | [ingest_runtime_glb](/crates/oxide-renderer/src/pcb3d/glb/ingest_runtime_glb.md) |
| calls | [request](/crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest/request.md) |
| calls | [emit_opaque_pass_preview](/crates/oxide-renderer/src/pcb3d/glb/emit_opaque_pass_preview.md) |
