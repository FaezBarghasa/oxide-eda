---
okf_version: "0.2"
type: Class
title: SymbolPinDetails
description: Extended pin fields surfaced on the Properties panel — flows
resource: crates/oxide-app/src/panels/symbol_context.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/symbol_context/SymbolPinDetails
language: rust
---

# SymbolPinDetails

Extended pin fields surfaced on the Properties panel — flows

## Signature

```rust
pub struct SymbolPinDetails
```

## Decorators

- `derive(Debug, Clone, PartialEq)`

## Visibility

- `pub`

## Docstring

Extended pin fields surfaced on the Properties panel — flows
alongside [`SymbolPinSummary`] so the panel doesn't have to
reach back into the editor state to read them. Mirrors the
Altium SchLib Pin Properties layout.
[derive(Debug, Clone, PartialEq)]

## Methods

- `description`
- `function`
- `pin_package_length`
- `propagation_delay_ns`
- `designator_visible`
- `name_visible`
- `inside_symbol`
- `inside_edge_symbol`
- `outside_edge_symbol`
- `outside_symbol`
- `hidden`
- `locked`
- `part_number`

## Source
Lines 128–144 in `crates/oxide-app/src/panels/symbol_context.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symbol_context](/crates/oxide-app/src/panels/symbol_context.md) |
