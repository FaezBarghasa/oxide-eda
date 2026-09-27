---
okf_version: "0.2"
type: Function
title: pcb3d_runtime_glb_ingest_accepts_minimal_valid_payload_from_path
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
concept_id: crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest/pcb3d_runtime_glb_ingest_accepts_minimal_valid_payload_from_path
language: rust
---

# pcb3d_runtime_glb_ingest_accepts_minimal_valid_payload_from_path

[test]

## Signature

```rust
fn pcb3d_runtime_glb_ingest_accepts_minimal_valid_payload_from_path()
```

## Decorators

- `test`

## Docstring

[test]

## Source
Lines 145–158 in `crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pcb3d_runtime_glb_ingest](/crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest.md) |
| calls | [unique_temp_path](/crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest/unique_temp_path.md) |
| calls | [make_glb_with_json](/crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest/make_glb_with_json.md) |
| calls | [valid_minimal_json](/crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest/valid_minimal_json.md) |
| calls | [ingest_runtime_glb](/crates/oxide-renderer/src/pcb3d/glb/ingest_runtime_glb.md) |
| calls | [request](/crates/oxide-renderer/tests/pcb3d_runtime_glb_ingest/request.md) |
