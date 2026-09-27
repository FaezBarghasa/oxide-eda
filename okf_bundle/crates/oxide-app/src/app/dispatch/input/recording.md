---
okf_version: "0.2"
type: Function
title: recording
description: "A recorder mid-capture. `KeymapRecorderState` has no `Default` —"
resource: crates/oxide-app/src/app/dispatch/input.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/input/recording
language: rust
---

# recording

A recorder mid-capture. `KeymapRecorderState` has no `Default` —

## Signature

```rust
fn recording() -> crate::app::KeymapRecorderState
```

## Docstring

A recorder mid-capture. `KeymapRecorderState` has no `Default` —
it is always seeded from the binding being edited.

## Source
Lines 520–527 in `crates/oxide-app/src/app/dispatch/input.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [input](/crates/oxide-app/src/app/dispatch/input.md) |
| called_by | [a_detached_preferences_moves_the_recorder_with_it](/crates/oxide-app/src/app/dispatch/input/a_detached_preferences_moves_the_recorder_with_it.md) |
| called_by | [the_recorder_claims_every_stroke_while_open](/crates/oxide-app/src/app/dispatch/input/the_recorder_claims_every_stroke_while_open.md) |
| called_by | [the_recorder_claims_only_where_preferences_is_painted_inline](/crates/oxide-app/src/app/dispatch/input/the_recorder_claims_only_where_preferences_is_painted_inline.md) |
| called_by | [the_recorder_outranks_the_palette](/crates/oxide-app/src/app/dispatch/input/the_recorder_outranks_the_palette.md) |
| called_by | [the_recorder_swallows_what_it_cannot_express](/crates/oxide-app/src/app/dispatch/input/the_recorder_swallows_what_it_cannot_express.md) |
