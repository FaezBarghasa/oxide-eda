---
okf_version: "0.2"
type: Class
title: PinMapInlineState
description: Per-row inline pin-map editor state — which row is currently expanded
resource: crates/oxide-app/src/library/state/preview.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/state/preview/PinMapInlineState
language: rust
---

# PinMapInlineState

Per-row inline pin-map editor state — which row is currently expanded

## Signature

```rust
pub struct PinMapInlineState
```

## Decorators

- `derive(Debug, Clone, Default, PartialEq, Eq)`

## Visibility

- `pub`

## Docstring

Per-row inline pin-map editor state — which row is currently expanded
and the live buffer for the target pad-number input. The pin/pad
bindings themselves live on `ComponentRow::pin_map_overrides`; this
struct only holds the UI-only flags.
[derive(Debug, Clone, Default, PartialEq, Eq)]

## Methods

- `expanded_row`
- `override_buf`

## Source
Lines 137–144 in `crates/oxide-app/src/library/state/preview.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [preview](/crates/oxide-app/src/library/state/preview.md) |
