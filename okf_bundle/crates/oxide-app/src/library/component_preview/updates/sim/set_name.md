---
okf_version: "0.2"
type: Function
title: set_name
description: "Set the simulation model's name, touching its `updated` timestamp."
resource: crates/oxide-app/src/library/component_preview/updates/sim.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/component_preview/updates/sim/set_name
language: rust
---

# set_name

Set the simulation model's name, touching its `updated` timestamp.

## Signature

```rust
pub(super) fn set_name(state: &mut ComponentPreviewState, name: String)
```

## Visibility

- `pub(super)`

## Docstring

Set the simulation model's name, touching its `updated` timestamp.

## Source
Lines 56–62 in `crates/oxide-app/src/library/component_preview/updates/sim.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [sim](/crates/oxide-app/src/library/component_preview/updates/sim.md) |
