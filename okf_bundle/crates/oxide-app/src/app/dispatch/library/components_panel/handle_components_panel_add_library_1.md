---
okf_version: "0.2"
type: Function
title: handle_components_panel_add_library
description: "\"+ Add Library…\" — opens the `*.snxlib` directory picker,"
resource: crates/oxide-app/src/app/dispatch/library/components_panel.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/components_panel/handle_components_panel_add_library_1
language: rust
---

# handle_components_panel_add_library

"+ Add Library…" — opens the `*.snxlib` directory picker,

## Signature

```rust
pub(super) fn handle_components_panel_add_library(
        &mut self,
        source: crate::library::state::ComponentsMountSource,
    ) -> Task<Message>
```

## Visibility

- `pub(super)`

## Docstring

"+ Add Library…" — opens the `*.snxlib` directory picker,
tagging the result with the section it was opened against.

## Source
Lines 28–47 in `crates/oxide-app/src/app/dispatch/library/components_panel.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [components_panel](/crates/oxide-app/src/app/dispatch/library/components_panel.md) |
