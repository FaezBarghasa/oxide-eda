---
okf_version: "0.2"
type: Function
title: metric
resource: crates/oxide-widgets/src/passive_calculator/rkm_encoder.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-widgets"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-widgets/src/passive_calculator/rkm_encoder/metric
language: rust
---

# metric

## Signature

```rust
fn metric(
    label: &'a str,
    value: String,
    tokens: &'a ThemeTokens,
) -> Element<'a, RkmEncoderMessage>
```

## Type Parameters

- `'a`

## Source
Lines 336–349 in `crates/oxide-widgets/src/passive_calculator/rkm_encoder.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [rkm_encoder](/crates/oxide-widgets/src/passive_calculator/rkm_encoder.md) |
| called_by | [view](/crates/oxide-widgets/src/passive_calculator/rkm_encoder/view.md) |
