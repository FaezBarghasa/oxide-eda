---
okf_version: "0.2"
type: Function
title: handle_add_library_primitive
description: Right-click → Add New ▸ Symbol / Footprint. Resolves the
resource: crates/oxide-app/src/app/handlers/menu/file_commands.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/menu/file_commands/handle_add_library_primitive_1
language: rust
---

# handle_add_library_primitive

Right-click → Add New ▸ Symbol / Footprint. Resolves the

## Signature

```rust
fn handle_add_library_primitive(
        &mut self,
        kind: oxide_library::PrimitiveKind,
    ) -> Task<Message>
```

## Docstring

Right-click → Add New ▸ Symbol / Footprint. Resolves the
clicked library from `project_tree_context_menu`, mints an
empty primitive via the adapter (which writes the JSON file
under `<library>/symbols|footprints/<uuid>.snx{sym,fpt}` and
commits), refreshes the project tree so the new file appears,
and opens the file as a standalone primitive-editor tab.

## Source
Lines 167–219 in `crates/oxide-app/src/app/handlers/menu/file_commands.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [file_commands](/crates/oxide-app/src/app/handlers/menu/file_commands.md) |
