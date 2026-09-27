---
okf_version: "0.2"
type: Class
title: PinPadOverride
description: "Pin-to-pad override — empty list = default 1:1 binding by number string"
resource: crates/oxide-library/src/component.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/component/PinPadOverride
language: rust
---

# PinPadOverride

Pin-to-pad override — empty list = default 1:1 binding by number string

## Signature

```rust
pub struct PinPadOverride
```

## Decorators

- `derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)`

## Visibility

- `pub`

## Docstring

Pin-to-pad override — empty list = default 1:1 binding by number string
equality. Non-empty entries override specific pins.
[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]

## Methods

- `symbol_pin_number`
- `footprint_pad_number`

## Source
Lines 58–61 in `crates/oxide-library/src/component.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [component](/crates/oxide-library/src/component.md) |
