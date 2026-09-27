---
okf_version: "0.2"
type: Function
title: editing_oval_width_param_propagates_through_solve
description: "v0.24 Track A5 — editing the `width_<slug>` parameter via the"
resource: crates/oxide-app/tests/regression/library_pad_geometry.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/regression/library_pad_geometry/editing_oval_width_param_propagates_through_solve
language: rust
---

# editing_oval_width_param_propagates_through_solve

v0.24 Track A5 — editing the `width_<slug>` parameter via the

## Signature

```rust
fn editing_oval_width_param_propagates_through_solve()
```

## Decorators

- `test`

## Docstring

v0.24 Track A5 — editing the `width_<slug>` parameter via the
dispatcher (the same path the Properties-panel "Width" row drives)
rewrites the bound parameter and runs a solve cleanly. The
resolved parameter map reflects the new width so any future
constraint linking Line endpoints to `width` would see the
updated value; we assert the resolved value here as the surface
proxy for "endpoint reflects the new width".
[test]

## Source
Lines 730–822 in `crates/oxide-app/tests/regression/library_pad_geometry.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_pad_geometry](/crates/oxide-app/tests/regression/library_pad_geometry.md) |
| calls | [mirror_add_pad_to_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_add_pad_to_sketch.md) |
