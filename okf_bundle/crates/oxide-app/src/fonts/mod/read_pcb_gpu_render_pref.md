---
okf_version: "0.2"
type: Function
title: read_pcb_gpu_render_pref
description: Read the PCB GPU-render toggle. A saved value always wins; the
resource: crates/oxide-app/src/fonts/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/fonts/mod/read_pcb_gpu_render_pref
language: rust
---

# read_pcb_gpu_render_pref

Read the PCB GPU-render toggle. A saved value always wins; the

## Signature

```rust
pub fn read_pcb_gpu_render_pref() -> bool
```

## Visibility

- `pub`

## Docstring

Read the PCB GPU-render toggle. A saved value always wins; the
compile-time [`crate::feature_flags::PCB_GPU_RENDER_DEFAULT`] applies only
when the key is absent, so it is a factory default rather than a gate and
old prefs files stay compatible.

## Source
Lines 701–703 in `crates/oxide-app/src/fonts/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [fonts](/crates/oxide-app/src/fonts/mod.md) |
| calls | [read_pcb_gpu_render_pref_at](/crates/oxide-app/src/fonts/mod/read_pcb_gpu_render_pref_at.md) |
| calls | [prefs_path](/crates/oxide-app/src/fonts/mod/prefs_path.md) |
| called_by | [new](/crates/oxide-app/src/app/bootstrap/new/new.md) |
