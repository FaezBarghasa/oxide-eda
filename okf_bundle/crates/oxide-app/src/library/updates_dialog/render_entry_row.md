---
okf_version: "0.2"
type: Function
title: render_entry_row
resource: crates/oxide-app/src/library/updates_dialog.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/library/updates_dialog/render_entry_row
language: rust
---

# render_entry_row

## Signature

```rust
fn render_entry_row(
    entry: &'a LibraryUpdateEntry,
    text_c: iced::Color,
    muted: iced::Color,
    _border: iced::Color,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Source
Lines 281–332 in `crates/oxide-app/src/library/updates_dialog.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [updates_dialog](/crates/oxide-app/src/library/updates_dialog.md) |
| called_by | [view](/crates/oxide-app/src/library/updates_dialog/view.md) |
