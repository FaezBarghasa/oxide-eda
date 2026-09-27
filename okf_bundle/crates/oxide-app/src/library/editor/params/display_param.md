---
okf_version: "0.2"
type: Function
title: display_param
description: "Cheap text view of a `ParamValue` for fallback rendering when the"
resource: crates/oxide-app/src/library/editor/params.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/params/display_param
language: rust
---

# display_param

Cheap text view of a `ParamValue` for fallback rendering when the

## Signature

```rust
fn display_param(v: &ParamValue) -> String
```

## Docstring

Cheap text view of a `ParamValue` for fallback rendering when the
per-row buffer is empty. Replaces the previous `ParamValue::display`
method whose name collided with type inference.

## Source
Lines 423–430 in `crates/oxide-app/src/library/editor/params.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [params](/crates/oxide-app/src/library/editor/params.md) |
| called_by | [slot_input](/crates/oxide-app/src/library/editor/params/slot_input.md) |
