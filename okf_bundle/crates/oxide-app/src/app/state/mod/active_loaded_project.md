---
okf_version: "0.2"
type: Function
title: active_loaded_project
description: "Convenience: currently-active project. Returns `None` when the"
resource: crates/oxide-app/src/app/state/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/state/mod/active_loaded_project
language: rust
---

# active_loaded_project

Convenience: currently-active project. Returns `None` when the

## Signature

```rust
impl DocumentState { pub fn active_loaded_project(&self) -> Option<&LoadedProject> }
```

## Visibility

- `pub`

## Docstring

Convenience: currently-active project. Returns `None` when the
workspace is empty or no project has been made active yet.

## Source
Lines 655–657 in `crates/oxide-app/src/app/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/app/state/mod.md) |
