---
okf_version: "0.2"
type: Function
title: key_sketch_input
description: v0.24 Track D — Sketch-mode live numeric placement input.
resource: crates/oxide-app/src/library/editor/footprint/canvas/input/keys.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/input/keys/key_sketch_input_1
language: rust
---

# key_sketch_input

v0.24 Track D — Sketch-mode live numeric placement input.

## Signature

```rust
fn key_sketch_input(
        &self,
        key: &keyboard::Key,
        modifiers: &keyboard::Modifiers,
        typed_char: Option<char>,
    ) -> Option<canvas::Action<LibraryMessage>>
```

## Docstring

v0.24 Track D — Sketch-mode live numeric placement input.
Active only while a multi-click sketch tool has its first click
pending or a buffer is open, so digit keys outside Sketch mode
never get swallowed. Modifiers must be empty so global
shortcuts still reach the app dispatcher.

## Source
Lines 127–236 in `crates/oxide-app/src/library/editor/footprint/canvas/input/keys.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [keys](/crates/oxide-app/src/library/editor/footprint/canvas/input/keys.md) |
