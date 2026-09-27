---
okf_version: "0.2"
type: Function
title: form_grid_size_row
resource: crates/oxide-app/src/panels/properties_parameters/form_rows.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/panels/properties_parameters/form_rows/form_grid_size_row
language: rust
---

# form_grid_size_row

## Signature

```rust
pub fn form_grid_size_row(current_mm: f32, label_c: Color) -> Element<'static, PanelMsg>
```

## Decorators

- `expect(
    dead_code,
    reason = "grid-size preset row kept for panels still being migrated to the shared rows"
)`

## Visibility

- `pub`

## Source
Lines 253–290 in `crates/oxide-app/src/panels/properties_parameters/form_rows.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [form_rows](/crates/oxide-app/src/panels/properties_parameters/form_rows.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
