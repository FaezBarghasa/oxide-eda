---
okf_version: "0.2"
type: Function
title: load_glb_bytes
resource: crates/oxide-renderer/src/pcb3d/glb.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-renderer/src/pcb3d/glb/load_glb_bytes
language: rust
---

# load_glb_bytes

## Signature

```rust
fn load_glb_bytes(model_id: &str, source: &GlbSource) -> Result<Vec<u8>, RuntimeGlbIngestError>
```

## Source
Lines 162–188 in `crates/oxide-renderer/src/pcb3d/glb.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [glb](/crates/oxide-renderer/src/pcb3d/glb.md) |
| calls | [is_glb_path](/crates/oxide-renderer/src/pcb3d/glb/is_glb_path.md) |
| called_by | [ingest_runtime_glb](/crates/oxide-renderer/src/pcb3d/glb/ingest_runtime_glb.md) |
