---
okf_version: "0.2"
type: Function
title: tag_btn
resource: crates/oxide-app/src/panels/properties_parameters/general.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/panels/properties_parameters/general/tag_btn
language: rust
---

# tag_btn

## Signature

```rust
pub fn tag_btn(
    label: &str,
    filter: crate::active_bar::SelectionFilter,
    enabled: bool,
    hover_bg: Color,
) -> Element<'static, PanelMsg>
```

## Decorators

- `expect(
    dead_code,
    reason = "Altium-style selection-filter pill, not yet wired into a panel"
)`

## Visibility

- `pub`

## Source
Lines 663–704 in `crates/oxide-app/src/panels/properties_parameters/general.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [general](/crates/oxide-app/src/panels/properties_parameters/general.md) |
