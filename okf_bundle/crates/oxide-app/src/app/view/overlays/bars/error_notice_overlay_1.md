---
okf_version: "0.2"
type: Function
title: error_notice_overlay
description: Export-error modal — appears when PDF / netlist / BOM export
resource: crates/oxide-app/src/app/view/overlays/bars.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/overlays/bars/error_notice_overlay_1
language: rust
---

# error_notice_overlay

Export-error modal — appears when PDF / netlist / BOM export

## Signature

```rust
pub(in crate::app::view) fn error_notice_overlay(&self) -> Vec<Element<'_, Message>>
```

## Visibility

- `pub(in crate::app::view)`

## Docstring

Export-error modal — appears when PDF / netlist / BOM export
hits a user-actionable failure (write permission, invalid path,
empty schematic). Dismiss via OK button or clicking outside.
Pushes the dismiss backdrop then the error card.

## Source
Lines 57–65 in `crates/oxide-app/src/app/view/overlays/bars.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bars](/crates/oxide-app/src/app/view/overlays/bars.md) |
