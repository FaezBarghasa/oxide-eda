---
okf_version: "0.2"
type: Function
title: form_edit_row_f64
description: Numeric edit row used by the shape pre-placement form. Writes on
resource: crates/oxide-app/src/panels/widgets.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/widgets/form_edit_row_f64
language: rust
---

# form_edit_row_f64

Numeric edit row used by the shape pre-placement form. Writes on

## Signature

```rust
pub fn form_edit_row_f64(
    label: &'a str,
    value: f64,
    muted: Color,
    on_submit: impl Fn(f64) -> PanelMsg + 'a + Clone,
) -> Element<'a, PanelMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub`

## Docstring

Numeric edit row used by the shape pre-placement form. Writes on
submit — partial text mid-type doesn't panic via parse failure.

## Source
Lines 246–272 in `crates/oxide-app/src/panels/widgets.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [widgets](/crates/oxide-app/src/panels/widgets.md) |
| called_by | [view_pre_placement](/crates/oxide-app/src/panels/properties/view_pre_placement.md) |
