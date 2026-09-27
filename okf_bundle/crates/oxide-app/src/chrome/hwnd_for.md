---
okf_version: "0.2"
type: Function
title: hwnd_for
resource: crates/oxide-app/src/chrome.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/chrome/hwnd_for
language: rust
---

# hwnd_for

## Signature

```rust
fn hwnd_for(w: &dyn iced::window::Window) -> Option<HWND>
```

## Source
Lines 91–97 in `crates/oxide-app/src/chrome.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [chrome](/crates/oxide-app/src/chrome.md) |
| called_by | [set_rounded](/crates/oxide-app/src/chrome/set_rounded.md) |
