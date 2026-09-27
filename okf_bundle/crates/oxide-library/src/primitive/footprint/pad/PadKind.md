---
okf_version: "0.2"
type: Class
title: PadKind
description: Pad mounting style.
resource: crates/oxide-library/src/primitive/footprint/pad.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-library/src/primitive/footprint/pad/PadKind
language: rust
---

# PadKind

Pad mounting style.

## Signature

```rust
pub enum PadKind
```

## Decorators

- `derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)`
- `non_exhaustive`

## Visibility

- `pub`

## Docstring

Pad mounting style.

Variant names persist in PascalCase to preserve v1 / v2 fixture
compatibility — adding `rename_all = "snake_case"` would break
every existing footprint TOML.
[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
[non_exhaustive]

## Source
Lines 38–55 in `crates/oxide-library/src/primitive/footprint/pad.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad](/crates/oxide-library/src/primitive/footprint/pad.md) |
