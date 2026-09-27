---
okf_version: "0.2"
type: Function
title: remove_button
resource: crates/oxide-app/src/library/editor/supply.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/supply/remove_button
language: rust
---

# remove_button

## Signature

```rust
fn remove_button(
    msg: EditorMsg,
    text_c: iced::Color,
    border: iced::Color,
    address: &EditorAddress,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Source
Lines 571–605 in `crates/oxide-app/src/library/editor/supply.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [supply](/crates/oxide-app/src/library/editor/supply.md) |
| called_by | [alternate_row](/crates/oxide-app/src/library/editor/supply/alternate_row.md) |
| called_by | [listing_row](/crates/oxide-app/src/library/editor/supply/listing_row.md) |
