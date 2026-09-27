---
okf_version: "0.2"
type: Function
title: handle_components_panel_promote_to_global
description: Promote an Installed library to Global.
resource: crates/oxide-app/src/app/dispatch/library/components_panel.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/components_panel/handle_components_panel_promote_to_global
language: rust
---

# handle_components_panel_promote_to_global

Promote an Installed library to Global.

## Signature

```rust
impl Oxide { pub(super) fn handle_components_panel_promote_to_global(
        &mut self,
        path: std::path::PathBuf,
    ) -> Task<Message> }
```

## Visibility

- `pub(super)`

## Docstring

Promote an Installed library to Global.

## Source
Lines 107–130 in `crates/oxide-app/src/app/dispatch/library/components_panel.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [components_panel](/crates/oxide-app/src/app/dispatch/library/components_panel.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
| calls | [add_path](/crates/oxide-app/src/panels/components_panel/global_prefs/add_path.md) |
