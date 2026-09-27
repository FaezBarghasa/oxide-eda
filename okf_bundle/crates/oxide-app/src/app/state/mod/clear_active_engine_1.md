---
okf_version: "0.2"
type: Function
title: clear_active_engine
description: Drop the engine for the active path. Used when closing the
resource: crates/oxide-app/src/app/state/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/state/mod/clear_active_engine_1
language: rust
---

# clear_active_engine

Drop the engine for the active path. Used when closing the

## Signature

```rust
pub fn clear_active_engine(&mut self)
```

## Visibility

- `pub`

## Docstring

Drop the engine for the active path. Used when closing the
active tab — the tab's engine is gone, and `active_path` follows
to whichever tab becomes active next.

## Source
Lines 732–737 in `crates/oxide-app/src/app/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/app/state/mod.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
