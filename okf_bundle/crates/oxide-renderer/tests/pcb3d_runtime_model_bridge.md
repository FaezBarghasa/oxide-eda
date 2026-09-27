---
okf_version: "0.2"
type: Module
title: pcb3d_runtime_model_bridge
description: "Integration tests for runtime bridge: source model -> importer -> GLB ingest."
resource: crates/oxide-renderer/tests/pcb3d_runtime_model_bridge.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-renderer"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-renderer/tests/pcb3d_runtime_model_bridge
language: rust
---

# pcb3d_runtime_model_bridge

Integration tests for runtime bridge: source model -> importer -> GLB ingest.

## Docstring

Integration tests for runtime bridge: source model -> importer -> GLB ingest.

## Relationships

| Type | Target |
|------|--------|
| related | [bridge_request](/crates/oxide-renderer/tests/pcb3d_runtime_model_bridge/bridge_request.md) |
| related | [make_glb_with_json](/crates/oxide-renderer/tests/pcb3d_runtime_model_bridge/make_glb_with_json.md) |
| related | [valid_minimal_json](/crates/oxide-renderer/tests/pcb3d_runtime_model_bridge/valid_minimal_json.md) |
| related | [write_tier0_wrl](/crates/oxide-renderer/tests/pcb3d_runtime_model_bridge/write_tier0_wrl.md) |
| related | [write_tier0_step](/crates/oxide-renderer/tests/pcb3d_runtime_model_bridge/write_tier0_step.md) |
| related | [pcb3d_runtime_bridge_passes_through_glb_path_without_conversion](/crates/oxide-renderer/tests/pcb3d_runtime_model_bridge/pcb3d_runtime_bridge_passes_through_glb_path_without_conversion.md) |
| related | [pcb3d_runtime_bridge_accepts_glb_bytes](/crates/oxide-renderer/tests/pcb3d_runtime_model_bridge/pcb3d_runtime_bridge_accepts_glb_bytes.md) |
| related | [pcb3d_runtime_bridge_converts_vrml_to_glb_and_ingests](/crates/oxide-renderer/tests/pcb3d_runtime_model_bridge/pcb3d_runtime_bridge_converts_vrml_to_glb_and_ingests.md) |
| related | [pcb3d_runtime_bridge_converts_step_to_glb_and_ingests](/crates/oxide-renderer/tests/pcb3d_runtime_model_bridge/pcb3d_runtime_bridge_converts_step_to_glb_and_ingests.md) |
| related | [pcb3d_runtime_bridge_reports_import_failure_for_missing_source](/crates/oxide-renderer/tests/pcb3d_runtime_model_bridge/pcb3d_runtime_bridge_reports_import_failure_for_missing_source.md) |
