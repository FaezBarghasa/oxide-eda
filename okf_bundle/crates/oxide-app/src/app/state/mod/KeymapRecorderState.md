---
okf_version: "0.2"
type: Class
title: KeymapRecorderState
description: Chord-recorder overlay state for the Preferences ▸ Keyboard
resource: crates/oxide-app/src/app/state/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/state/mod/KeymapRecorderState
language: rust
---

# KeymapRecorderState

Chord-recorder overlay state for the Preferences ▸ Keyboard

## Signature

```rust
pub struct KeymapRecorderState
```

## Decorators

- `derive(Debug, Clone, PartialEq, Eq)`

## Visibility

- `pub`

## Docstring

Chord-recorder overlay state for the Preferences ▸ Keyboard
Shortcuts pane. Holds the binding being edited plus the strokes
captured so far. The keyboard subscription feeds raw key events
here (as `PrefMsg::KeymapRecorderKeyPressed`) while this is `Some`,
so recording never triggers a live command.
[derive(Debug, Clone, PartialEq, Eq)]

## Methods

- `command`
- `command_label`
- `context`
- `original_trigger`
- `strokes`
- `modifiers`
- `recording`

## Source
Lines 32–40 in `crates/oxide-app/src/app/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/app/state/mod.md) |
