---
okf_version: "0.2"
type: Function
title: input_target
description: Classify the window an input event landed in.
resource: crates/oxide-app/src/app/dispatch/input.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/input/input_target_1
language: rust
---

# input_target

Classify the window an input event landed in.

## Signature

```rust
pub(crate) fn input_target(&self, window: Option<iced::window::Id>) -> InputTarget
```

## Visibility

- `pub(crate)`

## Docstring

Classify the window an input event landed in.

The main window is deliberately absent from `ui_state.windows`
(only `ui_state.main_window_id` names it), so a miss maps to
[`InputTarget::Main`] — as does a window already dropped from the
map on a close frame, which is the answer the pre-#547 code gave.

## Source
Lines 175–188 in `crates/oxide-app/src/app/dispatch/input.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [input](/crates/oxide-app/src/app/dispatch/input.md) |
