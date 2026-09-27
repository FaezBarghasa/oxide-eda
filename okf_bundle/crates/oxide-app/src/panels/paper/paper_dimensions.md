---
okf_version: "0.2"
type: Function
title: paper_dimensions
description: "(width_mm, height_mm) for a paper size string."
resource: crates/oxide-app/src/panels/paper.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/panels/paper/paper_dimensions
language: rust
---

# paper_dimensions

(width_mm, height_mm) for a paper size string.

## Signature

```rust
pub fn paper_dimensions(size: &str) -> (f32, f32)
```

## Visibility

- `pub`

## Docstring

(width_mm, height_mm) for a paper size string.

## Source
Lines 77–91 in `crates/oxide-app/src/panels/paper.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [paper](/crates/oxide-app/src/panels/paper.md) |
| called_by | [apply_page_dimensions_to_canvas](/crates/oxide-app/src/app/handlers/dock/panel_controls/apply_page_dimensions_to_canvas.md) |
| called_by | [view_properties_general](/crates/oxide-app/src/panels/properties_parameters/general/view_properties_general.md) |
