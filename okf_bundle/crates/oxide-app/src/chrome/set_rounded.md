---
okf_version: "0.2"
type: Function
title: set_rounded
resource: crates/oxide-app/src/chrome.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/chrome/set_rounded
language: rust
---

# set_rounded

## Signature

```rust
pub(super) fn set_rounded(w: &dyn iced::window::Window)
```

## Visibility

- `pub(super)`

## Source
Lines 73–89 in `crates/oxide-app/src/chrome.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [chrome](/crates/oxide-app/src/chrome.md) |
| calls | [hwnd_for](/crates/oxide-app/src/chrome/hwnd_for.md) |
| called_by | [apply_rounded_corners](/crates/oxide-app/src/chrome/apply_rounded_corners.md) |
