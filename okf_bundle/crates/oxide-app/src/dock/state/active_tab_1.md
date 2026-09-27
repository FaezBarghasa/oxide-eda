---
okf_version: "0.2"
type: Function
title: active_tab
description: "Which tab is active in `position`. Test-facing: the field is"
resource: crates/oxide-app/src/dock/state.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/dock/state/active_tab_1
language: rust
---

# active_tab

Which tab is active in `position`. Test-facing: the field is

## Signature

```rust
pub(super) fn active_tab(&self, position: PanelPosition) -> usize
```

## Decorators

- `cfg(test)`

## Visibility

- `pub(super)`

## Docstring

Which tab is active in `position`. Test-facing: the field is
private and `view.rs` reads it through `super::`.
[cfg(test)]

## Source
Lines 332–334 in `crates/oxide-app/src/dock/state.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/dock/state.md) |
