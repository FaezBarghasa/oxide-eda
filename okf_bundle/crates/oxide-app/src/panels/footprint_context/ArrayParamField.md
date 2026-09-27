---
okf_version: "0.2"
type: Class
title: ArrayParamField
description: "v0.23 — Field discriminator for [`PanelMsg::FpEditorEditArrayParam`]."
resource: crates/oxide-app/src/panels/footprint_context.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/footprint_context/ArrayParamField
language: rust
---

# ArrayParamField

v0.23 — Field discriminator for [`PanelMsg::FpEditorEditArrayParam`].

## Signature

```rust
pub enum ArrayParamField
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq)`

## Visibility

- `pub`

## Docstring

v0.23 — Field discriminator for [`PanelMsg::FpEditorEditArrayParam`].
Each variant maps to a single text-input on one [`ArrayKindSummary`]
branch; the handler uses the variant to disambiguate the target
field when mutating the array in place.
[derive(Debug, Clone, Copy, PartialEq, Eq)]

## Source
Lines 366–380 in `crates/oxide-app/src/panels/footprint_context.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_context](/crates/oxide-app/src/panels/footprint_context.md) |
