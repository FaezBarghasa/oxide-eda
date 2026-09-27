---
okf_version: "0.2"
type: Function
title: recorder_visible_in
description: "Whether the chord recorder is on screen in `target`."
resource: crates/oxide-app/src/app/dispatch/input.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/input/recorder_visible_in
language: rust
---

# recorder_visible_in

Whether the chord recorder is on screen in `target`.

## Signature

```rust
impl Oxide { fn recorder_visible_in(&self, target: InputTarget) -> bool }
```

## Docstring

Whether the chord recorder is on screen in `target`.

The recorder lives inside the Preferences body, which renders
either as the in-window card (main window and every undocked tab)
or, once detached, only in its own OS window — `view/chrome.rs`
passes `preferences_keymap_recorder` to both. So "where the
recorder is visible" is exactly "where Preferences is painted".

## Source
Lines 290–298 in `crates/oxide-app/src/app/dispatch/input.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [input](/crates/oxide-app/src/app/dispatch/input.md) |
