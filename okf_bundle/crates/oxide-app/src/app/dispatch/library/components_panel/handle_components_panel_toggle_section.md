---
okf_version: "0.2"
type: Function
title: handle_components_panel_toggle_section
description: Toggle the collapse flag for the named section.
resource: crates/oxide-app/src/app/dispatch/library/components_panel.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/components_panel/handle_components_panel_toggle_section
language: rust
---

# handle_components_panel_toggle_section

Toggle the collapse flag for the named section.

## Signature

```rust
impl Oxide { pub(super) fn handle_components_panel_toggle_section(
        &mut self,
        src: crate::library::state::ComponentsMountSource,
    ) -> Task<Message> }
```

## Visibility

- `pub(super)`

## Docstring

Toggle the collapse flag for the named section.

## Source
Lines 12–24 in `crates/oxide-app/src/app/dispatch/library/components_panel.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [components_panel](/crates/oxide-app/src/app/dispatch/library/components_panel.md) |
