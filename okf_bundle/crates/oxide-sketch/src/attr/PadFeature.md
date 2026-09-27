---
okf_version: "0.2"
type: Class
title: PadFeature
description: "Mechanical feature applied to a pad on the named side. Altium's"
resource: crates/oxide-sketch/src/attr.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-sketch/src/attr/PadFeature
language: rust
---

# PadFeature

Mechanical feature applied to a pad on the named side. Altium's

## Signature

```rust
pub enum PadFeature
```

## Decorators

- `derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)`
- `serde(rename_all = "PascalCase")`

## Visibility

- `pub`

## Docstring

Mechanical feature applied to a pad on the named side. Altium's
Pad Features section exposes this exactly:
- `None`        — bare copper / no machining.
- `Counterbore` — flat-bottom recess machined around the hole
to seat a fastener head flush.
- `Countersink` — conical recess machined around the hole for
a flat-head fastener.

The earlier "Solder Bumps / Glue Dots / Adhesive Beads" variants
were factually wrong — those are mechanical-layer purposes
(Glue Points / Coating / Gold Plating) authored as separate
primitives on dedicated mechanical layers, not pad fields.
[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
[serde(rename_all = "PascalCase")]

## Source
Lines 104–109 in `crates/oxide-sketch/src/attr.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [attr](/crates/oxide-sketch/src/attr.md) |
