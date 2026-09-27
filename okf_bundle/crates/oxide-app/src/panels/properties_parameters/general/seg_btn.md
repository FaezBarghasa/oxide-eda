---
okf_version: "0.2"
type: Function
title: seg_btn
resource: crates/oxide-app/src/panels/properties_parameters/general.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/panels/properties_parameters/general/seg_btn
language: rust
---

# seg_btn

## Signature

```rust
pub fn seg_btn(
    label: &str,
    active: bool,
    msg: PanelMsg,
    active_bg: Color,
    text_active: Color,
    text_inactive: Color,
    hover_bg: Color,
    seg_border: Color,
) -> Element<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Decorators

- `expect(
    clippy::too_many_arguments,
    reason = "a general-properties row builder whose parameters are all independent"
)`

## Visibility

- `pub`

## Source
Lines 711–753 in `crates/oxide-app/src/panels/properties_parameters/general.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [general](/crates/oxide-app/src/panels/properties_parameters/general.md) |
