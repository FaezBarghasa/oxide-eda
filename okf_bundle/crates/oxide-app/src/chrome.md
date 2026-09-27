---
okf_version: "0.2"
type: Module
title: chrome
description: OS-specific chrome polish for the borderless main window.
resource: crates/oxide-app/src/chrome.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/chrome
language: rust
---

# chrome

OS-specific chrome polish for the borderless main window.

## Docstring

OS-specific chrome polish for the borderless main window.

The borderless main window (see `bootstrap.rs` — `decorations: false`)
opts out of every native adornment the OS would otherwise provide: no
title-bar, no system menu, and on Windows 11 no rounded corners or
drop-shadow either. This module re-adds the corner rounding in the
cleanest per-OS way:

- **Windows 11** (build 22000+): `DwmSetWindowAttribute` with
`DWMWA_WINDOW_CORNER_PREFERENCE = DWMWCP_ROUND`. DWM handles the
mask, anti-aliasing, and the small system drop-shadow — matches how
VS Code's borderless frame looks. Silently no-ops on Windows 10.
- **macOS / Linux**: not yet wired. macOS already rounds top-level
windows on its own; Linux is WM-dependent. A transparent-window +
rounded-container fallback can be layered on later without touching
the Windows path.

## Relationships

| Type | Target |
|------|--------|
| related | [apply_rounded_corners](/crates/oxide-app/src/chrome/apply_rounded_corners.md) |
| related | [apply_rounded_corners](/crates/oxide-app/src/chrome/apply_rounded_corners.md) |
| related | [start_window_drag](/crates/oxide-app/src/chrome/start_window_drag.md) |
| related | [start_window_resize](/crates/oxide-app/src/chrome/start_window_resize.md) |
| related | [set_rounded](/crates/oxide-app/src/chrome/set_rounded.md) |
| related | [hwnd_for](/crates/oxide-app/src/chrome/hwnd_for.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
