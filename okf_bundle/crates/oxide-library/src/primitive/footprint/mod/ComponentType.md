---
okf_version: "0.2"
type: Class
title: ComponentType
description: v0.21 — Altium-parity component type. Drives whether the part
resource: crates/oxide-library/src/primitive/footprint/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-library/src/primitive/footprint/mod/ComponentType
language: rust
---

# ComponentType

v0.21 — Altium-parity component type. Drives whether the part

## Signature

```rust
pub enum ComponentType
```

## Decorators

- `derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)`
- `serde(rename_all = "PascalCase")`

## Visibility

- `pub`

## Docstring

v0.21 — Altium-parity component type. Drives whether the part
appears in the BOM and whether its pads can short different nets
(Net Tie / Jumper).
[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
[serde(rename_all = "PascalCase")]

## Source
Lines 402–411 in `crates/oxide-library/src/primitive/footprint/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint](/crates/oxide-library/src/primitive/footprint/mod.md) |
