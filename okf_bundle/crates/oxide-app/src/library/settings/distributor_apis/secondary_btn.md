---
okf_version: "0.2"
type: Function
title: secondary_btn
resource: crates/oxide-app/src/library/settings/distributor_apis.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/settings/distributor_apis/secondary_btn
language: rust
---

# secondary_btn

## Signature

```rust
fn secondary_btn(
    label: &'a str,
    message: LibraryMessage,
    text_c: iced::Color,
    border: iced::Color,
) -> Element<'a, LibraryMessage>
```

## Type Parameters

- `'a`

## Source
Lines 265–286 in `crates/oxide-app/src/library/settings/distributor_apis.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [distributor_apis](/crates/oxide-app/src/library/settings/distributor_apis.md) |
| called_by | [view](/crates/oxide-app/src/library/settings/distributor_apis/view.md) |
