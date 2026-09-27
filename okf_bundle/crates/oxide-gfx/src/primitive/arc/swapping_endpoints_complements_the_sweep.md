---
okf_version: "0.2"
type: Function
title: swapping_endpoints_complements_the_sweep
description: "Sign/direction sanity: swapping start and end complements the"
resource: crates/oxide-gfx/src/primitive/arc.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-gfx"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-gfx/src/primitive/arc/swapping_endpoints_complements_the_sweep
language: rust
---

# swapping_endpoints_complements_the_sweep

Sign/direction sanity: swapping start and end complements the

## Signature

```rust
fn swapping_endpoints_complements_the_sweep()
```

## Decorators

- `test`

## Docstring

Sign/direction sanity: swapping start and end complements the
sweep to `360° - sweep` (mirrors the placement-commit swap in
`symbol/updates/mod.rs`, which relies on exactly this
relationship to preserve the user's intended short arc).
[test]

## Source
Lines 108–115 in `crates/oxide-gfx/src/primitive/arc.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [arc](/crates/oxide-gfx/src/primitive/arc.md) |
| calls | [ccw_wrapped_sweep_rad](/crates/oxide-gfx/src/primitive/arc/ccw_wrapped_sweep_rad.md) |
