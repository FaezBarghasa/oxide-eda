---
okf_version: "0.2"
type: Class
title: PadStackOverrides
description: Per-side mask + paste expansions and the tented flag. Each Option
resource: crates/oxide-sketch/src/attr.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-sketch/src/attr/PadStackOverrides
language: rust
---

# PadStackOverrides

Per-side mask + paste expansions and the tented flag. Each Option

## Signature

```rust
pub struct PadStackOverrides
```

## Decorators

- `derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Per-side mask + paste expansions and the tented flag. Each Option
expression overrides the rule-driven default at bake time. `None`
here means "use the rule-driven value" — same convention as
Altium's "Rule Expansion" cell.
[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]

## Methods

- `paste_top_expr`
- `paste_top_pct`
- `paste_bottom_expr`
- `paste_bottom_pct`
- `paste_top_enabled`
- `paste_bottom_enabled`
- `mask_top_expr`
- `mask_bottom_expr`
- `mask_top_tented`
- `mask_bottom_tented`
- `thermal_relief`
- `corner_radius_pct`

## Source
Lines 163–201 in `crates/oxide-sketch/src/attr.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [attr](/crates/oxide-sketch/src/attr.md) |
