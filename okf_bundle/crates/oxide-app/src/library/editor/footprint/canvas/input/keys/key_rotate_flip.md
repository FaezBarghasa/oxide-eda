---
okf_version: "0.2"
type: Function
title: key_rotate_flip
description: v0.26-G — Space (rotate 90°) / X (flip layer) on the selected
resource: crates/oxide-app/src/library/editor/footprint/canvas/input/keys.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/footprint/canvas/input/keys/key_rotate_flip
language: rust
---

# key_rotate_flip

v0.26-G — Space (rotate 90°) / X (flip layer) on the selected

## Signature

```rust
impl FootprintCanvas<'_> { fn key_rotate_flip(
        &self,
        key: &keyboard::Key,
        modifiers: &keyboard::Modifiers,
    ) -> Option<canvas::Action<LibraryMessage>> }
```

## Docstring

v0.26-G — Space (rotate 90°) / X (flip layer) on the selected
pad. Altium parity; only fires when there's a pad to act on so
the canvas doesn't swallow Space / X from sketch tools.

## Source
Lines 85–120 in `crates/oxide-app/src/library/editor/footprint/canvas/input/keys.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [keys](/crates/oxide-app/src/library/editor/footprint/canvas/input/keys.md) |
