---
okf_version: "0.2"
type: Function
title: thin_sep
description: "Thin 1px separator line. `pub(super)` so sibling modules"
resource: crates/oxide-app/src/panels/properties_parameters/form_rows.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/panels/properties_parameters/form_rows/thin_sep
language: rust
---

# thin_sep

Thin 1px separator line. `pub(super)` so sibling modules

## Signature

```rust
pub fn thin_sep(border_c: Color) -> Element<'a, M>
```

## Type Parameters

- `'a`
- `M: 'a`

## Visibility

- `pub`

## Docstring

Thin 1px separator line. `pub(super)` so sibling modules
(footprint_editor_properties, symbol_editor_properties, etc.)
extracted from this file can share the single implementation.

## Source
Lines 13–22 in `crates/oxide-app/src/panels/properties_parameters/form_rows.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [form_rows](/crates/oxide-app/src/panels/properties_parameters/form_rows.md) |
