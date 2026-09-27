---
okf_version: "0.2"
type: Function
title: apply_eco
description: Apply an ECO Report directly onto a PCB Board model.
resource: crates/oxide-net/src/eco.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T20:13:59Z"
concept_id: crates/oxide-net/src/eco/apply_eco_1
language: rust
---

# apply_eco

Apply an ECO Report directly onto a PCB Board model.

## Signature

```rust
pub fn apply_eco(board: &mut PcbBoard, report: &EcoReport)
```

## Visibility

- `pub`

## Docstring

Apply an ECO Report directly onto a PCB Board model.

## Source
Lines 208–273 in `crates/oxide-net/src/eco.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [eco](/crates/oxide-net/src/eco.md) |
| calls | [find](/crates/oxide-net/src/uf/find.md) |
