---
okf_version: "0.2"
type: Function
title: bundled_window_icon
description: "Load the 256×256 PNG bundled by `installer/build-icons.sh` into an"
resource: crates/oxide-app/src/app/bootstrap/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/bootstrap/mod/bundled_window_icon
language: rust
---

# bundled_window_icon

Load the 256×256 PNG bundled by `installer/build-icons.sh` into an

## Signature

```rust
fn bundled_window_icon() -> Option<iced::window::Icon>
```

## Docstring

Load the 256×256 PNG bundled by `installer/build-icons.sh` into an
[`iced::window::Icon`]. When `has_bundled_icon` isn't set (i.e. the PNG
hasn't been generated yet) this returns `None` and the window opens with
the platform default icon.

## Source
Lines 7–19 in `crates/oxide-app/src/app/bootstrap/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [bootstrap](/crates/oxide-app/src/app/bootstrap/mod.md) |
| called_by | [new](/crates/oxide-app/src/app/bootstrap/new/new.md) |
