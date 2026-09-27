---
okf_version: "0.2"
type: Function
title: property_label
description: "Wrap a property-row label `text` in a clipped fill-portion container."
resource: crates/oxide-app/src/panels/widgets.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/widgets/property_label
language: rust
---

# property_label

Wrap a property-row label `text` in a clipped fill-portion container.

## Signature

```rust
pub fn property_label(label: impl Into<String>, color: Color) -> Element<'a, M>
```

## Type Parameters

- `'a`
- `M: 'a`

## Visibility

- `pub`

## Docstring

Wrap a property-row label `text` in a clipped fill-portion container.
Plain `text(...).width(FillPortion(...)).wrapping(None)` lays out at
the text's intrinsic width and bleeds past the allotted column when
the panel is narrow — covering the value column or the panel edge.
This helper enforces the FillPortion bound and clips visual overflow
inside it. Used by every `form_*_row` and inline property row.

## Source
Lines 219–229 in `crates/oxide-app/src/panels/widgets.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [widgets](/crates/oxide-app/src/panels/widgets.md) |
