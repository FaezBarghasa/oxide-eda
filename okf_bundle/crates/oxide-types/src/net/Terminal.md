---
okf_version: "0.2"
type: Class
title: Terminal
description: "One pin instance connected to a net: the placed symbol's `uuid`, its"
resource: crates/oxide-types/src/net.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T04:59:29Z"
concept_id: crates/oxide-types/src/net/Terminal
language: rust
---

# Terminal

One pin instance connected to a net: the placed symbol's `uuid`, its

## Signature

```rust
pub struct Terminal
```

## Decorators

- `derive(Debug, Clone, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

One pin instance connected to a net: the placed symbol's `uuid`, its
reference designator (`R1`, `U3`), and the pin identifier (number or name).

`symbol` disambiguates terminals a bare reference string collapses —
unannotated `R?` and duplicate designators (the same refdes on two sheet
occurrences) — and links the terminal back to the placed symbol. `reference`
and `pin` stay for exporters and display.
[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

## Methods

- `symbol`
- `reference`
- `pin`
- `internal_delay_ps`

## Source
Lines 58–66 in `crates/oxide-types/src/net.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [net](/crates/oxide-types/src/net.md) |
