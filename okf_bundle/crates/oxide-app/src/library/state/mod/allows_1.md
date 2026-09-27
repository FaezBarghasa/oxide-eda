---
okf_version: "0.2"
type: Function
title: allows
description: "Whether a row in `state` should render under this filter."
resource: crates/oxide-app/src/library/state/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/state/mod/allows_1
language: rust
---

# allows

Whether a row in `state` should render under this filter.

## Signature

```rust
pub fn allows(self, state: oxide_library::LifecycleState) -> bool
```

## Visibility

- `pub`

## Docstring

Whether a row in `state` should render under this filter.
Deprecated rows render in the default filter but render tinted
(see `lifecycle_dot_color`). `LifecycleState` is
`#[non_exhaustive]` so the match falls through to the default
(active-but-not-preferred) bucket for any future variant.

## Source
Lines 126–141 in `crates/oxide-app/src/library/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/library/state/mod.md) |
