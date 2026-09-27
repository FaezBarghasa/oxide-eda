---
okf_version: "0.2"
type: Function
title: message_card
description: Single muted card used by the empty/no-repo/loading/error states.
resource: crates/oxide-app/src/panels/history.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/history/message_card
language: rust
---

# message_card

Single muted card used by the empty/no-repo/loading/error states.

## Signature

```rust
fn message_card(
    msg: impl Into<String>,
    muted: iced::Color,
    border_c: iced::Color,
    bg: Option<iced::Background>,
) -> Element<'a, M>
```

## Type Parameters

- `'a`
- `M: 'a`

## Docstring

Single muted card used by the empty/no-repo/loading/error states.

Takes an owned message rather than `&'a str` so the error state can
pass a formatted string; the static callers are unchanged.

## Source
Lines 153–173 in `crates/oxide-app/src/panels/history.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [history](/crates/oxide-app/src/panels/history.md) |
| called_by | [view_history](/crates/oxide-app/src/panels/history/view_history.md) |
