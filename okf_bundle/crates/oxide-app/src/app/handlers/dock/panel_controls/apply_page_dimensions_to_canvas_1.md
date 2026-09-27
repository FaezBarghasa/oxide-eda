---
okf_version: "0.2"
type: Function
title: apply_page_dimensions_to_canvas
description: Push the effective paper dimensions from PanelContext into the canvas so
resource: crates/oxide-app/src/app/handlers/dock/panel_controls.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:39:58Z"
concept_id: crates/oxide-app/src/app/handlers/dock/panel_controls/apply_page_dimensions_to_canvas_1
language: rust
---

# apply_page_dimensions_to_canvas

Push the effective paper dimensions from PanelContext into the canvas so

## Signature

```rust
pub(crate) fn apply_page_dimensions_to_canvas(&mut self)
```

## Visibility

- `pub(crate)`

## Docstring

Push the effective paper dimensions from PanelContext into the canvas so
the background / grid track Page Options changes immediately. Also
called from the document-load path so an opened sheet's stored paper
size drives the drawn sheet, not the previous tab's leftovers.

## Source
Lines 10–19 in `crates/oxide-app/src/app/handlers/dock/panel_controls.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [panel_controls](/crates/oxide-app/src/app/handlers/dock/panel_controls.md) |
| calls | [paper_dimensions](/crates/oxide-app/src/panels/paper/paper_dimensions.md) |
