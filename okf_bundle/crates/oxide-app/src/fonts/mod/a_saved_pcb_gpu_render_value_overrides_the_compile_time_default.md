---
okf_version: "0.2"
type: Function
title: a_saved_pcb_gpu_render_value_overrides_the_compile_time_default
description: "`PCB_GPU_RENDER_DEFAULT` is a factory default, not a gate: a saved"
resource: crates/oxide-app/src/fonts/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/fonts/mod/a_saved_pcb_gpu_render_value_overrides_the_compile_time_default
language: rust
---

# a_saved_pcb_gpu_render_value_overrides_the_compile_time_default

`PCB_GPU_RENDER_DEFAULT` is a factory default, not a gate: a saved

## Signature

```rust
fn a_saved_pcb_gpu_render_value_overrides_the_compile_time_default()
```

## Decorators

- `test`

## Docstring

`PCB_GPU_RENDER_DEFAULT` is a factory default, not a gate: a saved
value wins over it in both directions. The module doc in
`feature_flags` used to claim the opposite, which would make someone
reach for that const to disable the feature for everyone and ship a
release where it was still on.
[test]

## Source
Lines 899–912 in `crates/oxide-app/src/fonts/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [fonts](/crates/oxide-app/src/fonts/mod.md) |
| calls | [temp_prefs](/crates/oxide-app/src/fonts/mod/temp_prefs.md) |
