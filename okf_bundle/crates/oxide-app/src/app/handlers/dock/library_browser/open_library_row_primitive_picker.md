---
okf_version: "0.2"
type: Function
title: open_library_row_primitive_picker
description: Open the primitive picker for the row described by the active
resource: crates/oxide-app/src/app/handlers/dock/library_browser.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/library_browser/open_library_row_primitive_picker
language: rust
---

# open_library_row_primitive_picker

Open the primitive picker for the row described by the active

## Signature

```rust
impl Oxide { fn open_library_row_primitive_picker(&mut self, kind: oxide_library::PrimitiveKind) }
```

## Docstring

Open the primitive picker for the row described by the active
`panel_ctx.library_row_detail`. Wired by the Properties-panel
Pick Symbol / Pick Footprint buttons.

## Source
Lines 42–57 in `crates/oxide-app/src/app/handlers/dock/library_browser.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [library_browser](/crates/oxide-app/src/app/handlers/dock/library_browser.md) |
