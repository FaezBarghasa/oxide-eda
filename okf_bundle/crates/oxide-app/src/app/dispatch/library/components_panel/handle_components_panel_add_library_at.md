---
okf_version: "0.2"
type: Function
title: handle_components_panel_add_library_at
description: Result of the Add Library file dialog — mount and record it in
resource: crates/oxide-app/src/app/dispatch/library/components_panel.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/components_panel/handle_components_panel_add_library_at
language: rust
---

# handle_components_panel_add_library_at

Result of the Add Library file dialog — mount and record it in

## Signature

```rust
impl Oxide { pub(super) fn handle_components_panel_add_library_at(
        &mut self,
        source: crate::library::state::ComponentsMountSource,
        path: Option<std::path::PathBuf>,
    ) -> Task<Message> }
```

## Visibility

- `pub(super)`

## Docstring

Result of the Add Library file dialog — mount and record it in
the matching source bucket.

## Source
Lines 51–104 in `crates/oxide-app/src/app/dispatch/library/components_panel.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [components_panel](/crates/oxide-app/src/app/dispatch/library/components_panel.md) |
| calls | [add_path](/crates/oxide-app/src/panels/components_panel/global_prefs/add_path.md) |
