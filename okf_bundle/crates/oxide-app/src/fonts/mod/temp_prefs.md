---
okf_version: "0.2"
type: Function
title: temp_prefs
resource: crates/oxide-app/src/fonts/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/fonts/mod/temp_prefs
language: rust
---

# temp_prefs

## Signature

```rust
fn temp_prefs(contents: &str) -> (tempfile::TempDir, std::path::PathBuf)
```

## Source
Lines 883–891 in `crates/oxide-app/src/fonts/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [fonts](/crates/oxide-app/src/fonts/mod.md) |
| called_by | [a_saved_pcb_gpu_render_value_overrides_the_compile_time_default](/crates/oxide-app/src/fonts/mod/a_saved_pcb_gpu_render_value_overrides_the_compile_time_default.md) |
| called_by | [an_absent_pcb_gpu_render_key_falls_back_to_the_compile_time_default](/crates/oxide-app/src/fonts/mod/an_absent_pcb_gpu_render_key_falls_back_to_the_compile_time_default.md) |
