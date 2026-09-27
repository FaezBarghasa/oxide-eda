---
okf_version: "0.2"
type: Function
title: handle_components_panel_add_to_project
description: "\"Add to Project\" on a Components Panel row. Stage 9 stub."
resource: crates/oxide-app/src/app/dispatch/library/components_panel.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/components_panel/handle_components_panel_add_to_project
language: rust
---

# handle_components_panel_add_to_project

"Add to Project" on a Components Panel row. Stage 9 stub.

## Signature

```rust
impl Oxide { pub(super) fn handle_components_panel_add_to_project(
        &mut self,
        library_path: std::path::PathBuf,
    ) -> Task<Message> }
```

## Visibility

- `pub(super)`

## Docstring

"Add to Project" on a Components Panel row. Stage 9 stub.

## Source
Lines 142–152 in `crates/oxide-app/src/app/dispatch/library/components_panel.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [components_panel](/crates/oxide-app/src/app/dispatch/library/components_panel.md) |
