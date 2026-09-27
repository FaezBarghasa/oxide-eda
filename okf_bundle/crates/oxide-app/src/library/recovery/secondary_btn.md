---
okf_version: "0.2"
type: Function
title: secondary_btn
resource: crates/oxide-app/src/library/recovery.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/recovery/secondary_btn
language: rust
---

# secondary_btn

## Signature

```rust
fn secondary_btn(
    label: &'a str,
    message: LibraryMessage,
    text_color: iced::Color,
    border: iced::Color,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Source
Lines 392–413 in `crates/oxide-app/src/library/recovery.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [recovery](/crates/oxide-app/src/library/recovery.md) |
| called_by | [git_missing_view](/crates/oxide-app/src/library/recovery/git_missing_view.md) |
