---
okf_version: "0.2"
type: Function
title: set_pin_node
description: "Set (or clear, when blank) the default node mapping for one pin."
resource: crates/oxide-app/src/library/component_preview/updates/sim.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/component_preview/updates/sim/set_pin_node
language: rust
---

# set_pin_node

Set (or clear, when blank) the default node mapping for one pin.

## Signature

```rust
pub(super) fn set_pin_node(state: &mut ComponentPreviewState, pin_number: String, value: String)
```

## Visibility

- `pub(super)`

## Docstring

Set (or clear, when blank) the default node mapping for one pin.

## Source
Lines 81–92 in `crates/oxide-app/src/library/component_preview/updates/sim.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sim](/crates/oxide-app/src/library/component_preview/updates/sim.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
| called_by | [apply_inline_edit](/crates/oxide-app/src/library/component_preview/updates/mod/apply_inline_edit.md) |
