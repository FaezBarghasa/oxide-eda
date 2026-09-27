---
okf_version: "0.2"
type: Function
title: pin_rotation_deg
description: "See the module doc's \"Pin rotation convention\" section for why this"
resource: crates/oxide-library/src/primitive/symbol/to_lib_symbol.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/primitive/symbol/to_lib_symbol/pin_rotation_deg
language: rust
---

# pin_rotation_deg

See the module doc's "Pin rotation convention" section for why this

## Signature

```rust
fn pin_rotation_deg(orientation: PinOrientation) -> f64
```

## Docstring

See the module doc's "Pin rotation convention" section for why this
is the identity on angle — both sides already agree.

`PinOrientation` is `#[non_exhaustive]` for downstream crates, but
this match lives in the crate that defines it, so it stays exhaustive
with no wildcard arm — a future variant added to the enum fails this
match at compile time instead of silently falling through.

## Source
Lines 140–147 in `crates/oxide-library/src/primitive/symbol/to_lib_symbol.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [to_lib_symbol](/crates/oxide-library/src/primitive/symbol/to_lib_symbol.md) |
| called_by | [lib_pin_from](/crates/oxide-library/src/primitive/symbol/to_lib_symbol/lib_pin_from.md) |
