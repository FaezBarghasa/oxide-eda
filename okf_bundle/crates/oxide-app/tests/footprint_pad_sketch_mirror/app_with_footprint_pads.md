---
okf_version: "0.2"
type: Function
title: app_with_footprint_pads
description: "One app with `count` default pads on a footprint editor — the"
resource: crates/oxide-app/tests/footprint_pad_sketch_mirror.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/footprint_pad_sketch_mirror/app_with_footprint_pads
language: rust
---

# app_with_footprint_pads

One app with `count` default pads on a footprint editor — the

## Signature

```rust
fn app_with_footprint_pads(stem: &str, count: usize) -> (oxide_app::app::Oxide, PathBuf)
```

## Docstring

One app with `count` default pads on a footprint editor — the
local twin of `regression.rs`'s `fixture_footprint_with_pads`,
carried along with the test that needs it.

## Source
Lines 394–413 in `crates/oxide-app/tests/footprint_pad_sketch_mirror.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_pad_sketch_mirror](/crates/oxide-app/tests/footprint_pad_sketch_mirror.md) |
| called_by | [v026e_paste_does_not_alias_template_shape_params](/crates/oxide-app/tests/footprint_pad_sketch_mirror/v026e_paste_does_not_alias_template_shape_params.md) |
