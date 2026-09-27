---
okf_version: "0.2"
type: Function
title: sanitize_node_name
description: Sanitize net names into valid SPICE node identifiers (mapping GND variants to 0).
resource: crates/oxide-sim/src/deck/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:49:22Z"
concept_id: crates/oxide-sim/src/deck/mod/sanitize_node_name
language: rust
---

# sanitize_node_name

Sanitize net names into valid SPICE node identifiers (mapping GND variants to 0).

## Signature

```rust
fn sanitize_node_name(name: &str) -> String
```

## Docstring

Sanitize net names into valid SPICE node identifiers (mapping GND variants to 0).

## Source
Lines 284–295 in `crates/oxide-sim/src/deck/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [deck](/crates/oxide-sim/src/deck/mod.md) |
| called_by | [build](/crates/oxide-sim/src/deck/mod/build.md) |
| called_by | [build_pin_to_node_map](/crates/oxide-sim/src/deck/mod/build_pin_to_node_map.md) |
