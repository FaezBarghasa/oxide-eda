---
okf_version: "0.2"
type: Function
title: view_print_preview
description: Print Preview overlay. Shows thumbnails of every rendered page on
resource: crates/oxide-app/src/app/view/print_preview.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/print_preview/view_print_preview
language: rust
---

# view_print_preview

Print Preview overlay. Shows thumbnails of every rendered page on

## Signature

```rust
impl Oxide { pub(super) fn view_print_preview(&self) -> Element<'_, Message> }
```

## Visibility

- `pub(super)`

## Docstring

Print Preview overlay. Shows thumbnails of every rendered page on
the left, the selected page full-size on the right, with Export PDF
and Close buttons at the bottom. Triggered by File → Print Preview
(Ctrl+P) and File → Export PDF; disappears on Close or when the
export completes. In-window flavour wraps the body in `wrap_modal`
for backdrop + drag-to-position.

## Source
Lines 205–221 in `crates/oxide-app/src/app/view/print_preview.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [print_preview](/crates/oxide-app/src/app/view/print_preview.md) |
| calls | [wrap_modal](/crates/oxide-app/src/app/view/dialogs/widgets/wrap_modal.md) |
