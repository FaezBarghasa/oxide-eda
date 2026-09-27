---
okf_version: "0.2"
type: Function
title: unique_name_in
description: "Pick a filename under `dir` that doesn't collide with anything on"
resource: crates/oxide-app/src/app/handlers/dock/project_navigation/add.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/project_navigation/add/unique_name_in
language: rust
---

# unique_name_in

Pick a filename under `dir` that doesn't collide with anything on

## Signature

```rust
fn unique_name_in(dir: &std::path::Path, base: &str, ext: &str) -> String
```

## Docstring

Pick a filename under `dir` that doesn't collide with anything on
disk. Tries `<base>.<ext>`, then `<base>2.<ext>`, `<base>3.<ext>`,
etc. Used to seed the Add-New-Schematic Save-As dialog so the user
doesn't have to dodge an existing file by hand.

## Source
Lines 388–400 in `crates/oxide-app/src/app/handlers/dock/project_navigation/add.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [add](/crates/oxide-app/src/app/handlers/dock/project_navigation/add.md) |
| called_by | [add_new_schematic](/crates/oxide-app/src/app/handlers/dock/project_navigation/add/add_new_schematic.md) |
| called_by | [add_project_footprint_library](/crates/oxide-app/src/app/handlers/dock/project_navigation/add/add_project_footprint_library.md) |
| called_by | [add_project_symbol_library](/crates/oxide-app/src/app/handlers/dock/project_navigation/add/add_project_symbol_library.md) |
