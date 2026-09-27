---
okf_version: "0.2"
type: Class
title: Netlist
description: "The authoritative netlist: every net derived from a schematic. This is the"
resource: crates/oxide-types/src/net.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-19T04:59:29Z"
concept_id: crates/oxide-types/src/net/Netlist
language: rust
---

# Netlist

The authoritative netlist: every net derived from a schematic. This is the

## Signature

```rust
pub struct Netlist
```

## Decorators

- `derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

The authoritative netlist: every net derived from a schematic. This is the
single connectivity source the net-flood UI, the ratsnest, PCB net
assignment, and the netlist exporter are meant to read — replacing the
ad-hoc union-find copies scattered across the app (ADR-0001 A3.1).
[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]

## Methods

- `nets`
- `xsignals`

## Source
Lines 161–165 in `crates/oxide-types/src/net.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [net](/crates/oxide-types/src/net.md) |
