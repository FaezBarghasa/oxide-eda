---
okf_version: "0.2"
type: Function
title: set_enabled
description: "Enable or disable the row's simulation model."
resource: crates/oxide-app/src/library/component_preview/updates/sim.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/component_preview/updates/sim/set_enabled
language: rust
---

# set_enabled

Enable or disable the row's simulation model.

## Signature

```rust
pub(super) fn set_enabled(state: &mut ComponentPreviewState, enabled: bool)
```

## Visibility

- `pub(super)`

## Docstring

Enable or disable the row's simulation model.

Enabling mints a fresh SPICE3 `SimModel` (and an editor buffer) when
the row has none; disabling clears the model, its binding, and the
buffer.

## Source
Lines 14–44 in `crates/oxide-app/src/library/component_preview/updates/sim.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sim](/crates/oxide-app/src/library/component_preview/updates/sim.md) |
| called_by | [apply_inline_edit](/crates/oxide-app/src/library/component_preview/updates/mod/apply_inline_edit.md) |
