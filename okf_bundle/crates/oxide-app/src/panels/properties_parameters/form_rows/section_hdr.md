---
okf_version: "0.2"
type: Function
title: section_hdr
description: "Section header: bold label + separator line."
resource: crates/oxide-app/src/panels/properties_parameters/form_rows.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/panels/properties_parameters/form_rows/section_hdr
language: rust
---

# section_hdr

Section header: bold label + separator line.

## Signature

```rust
pub fn section_hdr(title: &str, text_c: Color, border_c: Color) -> Column<'a, M>
```

## Type Parameters

- `'a`
- `M: 'a`

## Visibility

- `pub`

## Docstring

Section header: bold label + separator line.

## Source
Lines 25–39 in `crates/oxide-app/src/panels/properties_parameters/form_rows.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [form_rows](/crates/oxide-app/src/panels/properties_parameters/form_rows.md) |
| called_by | [view_components](/crates/oxide-app/src/panels/components/view_components.md) |
| called_by | [view_properties_parameters](/crates/oxide-app/src/panels/properties_parameters/general/view_properties_parameters.md) |
