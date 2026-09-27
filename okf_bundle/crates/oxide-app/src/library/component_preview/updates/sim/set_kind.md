---
okf_version: "0.2"
type: Function
title: set_kind
description: "Set the simulation model's kind, touching its `updated` timestamp."
resource: crates/oxide-app/src/library/component_preview/updates/sim.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/component_preview/updates/sim/set_kind
language: rust
---

# set_kind

Set the simulation model's kind, touching its `updated` timestamp.

## Signature

```rust
pub(super) fn set_kind(state: &mut ComponentPreviewState, kind: oxide_library::SimKind)
```

## Visibility

- `pub(super)`

## Docstring

Set the simulation model's kind, touching its `updated` timestamp.

## Source
Lines 47–53 in `crates/oxide-app/src/library/component_preview/updates/sim.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sim](/crates/oxide-app/src/library/component_preview/updates/sim.md) |
| called_by | [apply_inline_edit](/crates/oxide-app/src/library/component_preview/updates/mod/apply_inline_edit.md) |
