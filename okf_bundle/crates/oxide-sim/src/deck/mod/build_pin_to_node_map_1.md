---
okf_version: "0.2"
type: Function
title: build_pin_to_node_map
resource: crates/oxide-sim/src/deck/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sim"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:49:22Z"
concept_id: crates/oxide-sim/src/deck/mod/build_pin_to_node_map_1
language: rust
---

# build_pin_to_node_map

## Signature

```rust
fn build_pin_to_node_map(&self) -> HashMap<(String, String), String>
```

## Source
Lines 259–270 in `crates/oxide-sim/src/deck/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [deck](/crates/oxide-sim/src/deck/mod.md) |
| calls | [sanitize_node_name](/crates/oxide-sim/src/deck/mod/sanitize_node_name.md) |
