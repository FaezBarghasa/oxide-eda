---
okf_version: "0.2"
type: Function
title: dispatch_grid_properties_message
description: "Grid Properties dialog handler (namespaced family, ADR-0001 D3)."
resource: crates/oxide-app/src/app/dispatch/ui.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/ui/dispatch_grid_properties_message
language: rust
---

# dispatch_grid_properties_message

Grid Properties dialog handler (namespaced family, ADR-0001 D3).

## Signature

```rust
impl Oxide { pub(crate) fn dispatch_grid_properties_message(
        &mut self,
        msg: GridPropertiesMsg,
    ) -> Task<Message> }
```

## Visibility

- `pub(crate)`

## Docstring

Grid Properties dialog handler (namespaced family, ADR-0001 D3).

## Source
Lines 224–342 in `crates/oxide-app/src/app/dispatch/ui.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ui](/crates/oxide-app/src/app/dispatch/ui.md) |
