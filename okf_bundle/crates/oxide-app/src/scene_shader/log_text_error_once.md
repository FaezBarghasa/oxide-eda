---
okf_version: "0.2"
type: Function
title: log_text_error_once
description: Report the first glyph-atlas failure of each kind and stay silent after.
resource: crates/oxide-app/src/scene_shader.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/scene_shader/log_text_error_once
language: rust
---

# log_text_error_once

Report the first glyph-atlas failure of each kind and stay silent after.

## Signature

```rust
fn log_text_error_once(
    flag: &AtomicBool,
    surface: &str,
    stage: &str,
    error: impl std::fmt::Display,
)
```

## Docstring

Report the first glyph-atlas failure of each kind and stay silent after.

A dropped frame of text beats panicking the render thread, but a swallowed
failure that never reaches the Messages panel is not reporting either. This
runs once per frame, so it must not log at frame rate.

## Source
Lines 151–163 in `crates/oxide-app/src/scene_shader.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [scene_shader](/crates/oxide-app/src/scene_shader.md) |
| called_by | [draw](/crates/oxide-app/src/scene_shader/draw.md) |
| called_by | [prepare](/crates/oxide-app/src/scene_shader/prepare.md) |
