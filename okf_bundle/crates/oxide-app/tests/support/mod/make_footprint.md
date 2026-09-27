---
okf_version: "0.2"
type: Function
title: make_footprint
description: "A SOIC-16 footprint: 16 SMD pads, a courtyard polygon, silkscreen"
resource: crates/oxide-app/tests/support/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/support/mod/make_footprint
language: rust
---

# make_footprint

A SOIC-16 footprint: 16 SMD pads, a courtyard polygon, silkscreen

## Signature

```rust
fn make_footprint(index: usize) -> Footprint
```

## Docstring

A SOIC-16 footprint: 16 SMD pads, a courtyard polygon, silkscreen
outline + pin-1 marker, and a fab-layer label.

## Source
Lines 175–276 in `crates/oxide-app/tests/support/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [support](/crates/oxide-app/tests/support/mod.md) |
| calls | [sample_params](/crates/oxide-app/tests/support/mod/sample_params.md) |
| called_by | [append_component](/crates/oxide-app/tests/support/mod/append_component.md) |
| called_by | [generate_library](/crates/oxide-app/tests/support/mod/generate_library.md) |
