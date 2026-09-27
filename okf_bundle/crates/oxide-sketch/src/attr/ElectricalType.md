---
okf_version: "0.2"
type: Class
title: ElectricalType
description: "Pad-level electrical-type flag. Altium's PCB pad Properties shows"
resource: crates/oxide-sketch/src/attr.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-sketch/src/attr/ElectricalType
language: rust
---

# ElectricalType

Pad-level electrical-type flag. Altium's PCB pad Properties shows

## Signature

```rust
pub enum ElectricalType
```

## Decorators

- `derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)`
- `serde(rename_all = "PascalCase")`

## Visibility

- `pub`

## Docstring

Pad-level electrical-type flag. Altium's PCB pad Properties shows
only three values (`Load / Source / Terminator`) — distinct from
the schematic Pin's eight-value enum. Used by Signal-Integrity
rules and the testpoint auto-classifier; defaults to `Load`.
[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
[serde(rename_all = "PascalCase")]

## Source
Lines 59–64 in `crates/oxide-sketch/src/attr.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [attr](/crates/oxide-sketch/src/attr.md) |
