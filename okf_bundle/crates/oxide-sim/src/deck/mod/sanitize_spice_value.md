---
okf_version: "0.2"
type: Function
title: sanitize_spice_value
description: "Convert user schematic values (e.g. 10k, 100n, 10u, 1MEG) to valid SPICE strings."
resource: crates/oxide-sim/src/deck/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:49:22Z"
concept_id: crates/oxide-sim/src/deck/mod/sanitize_spice_value
language: rust
---

# sanitize_spice_value

Convert user schematic values (e.g. 10k, 100n, 10u, 1MEG) to valid SPICE strings.

## Signature

```rust
fn sanitize_spice_value(val: &str) -> String
```

## Docstring

Convert user schematic values (e.g. 10k, 100n, 10u, 1MEG) to valid SPICE strings.

## Source
Lines 302–320 in `crates/oxide-sim/src/deck/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [deck](/crates/oxide-sim/src/deck/mod.md) |
| called_by | [build](/crates/oxide-sim/src/deck/mod/build.md) |
