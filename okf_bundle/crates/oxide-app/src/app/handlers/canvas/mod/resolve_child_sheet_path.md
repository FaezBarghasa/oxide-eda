---
okf_version: "0.2"
type: Function
title: resolve_child_sheet_path
resource: crates/oxide-app/src/app/handlers/canvas/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/canvas/mod/resolve_child_sheet_path
language: rust
---

# resolve_child_sheet_path

## Signature

```rust
impl Oxide { fn resolve_child_sheet_path(&self, child_filename: &str) -> Option<std::path::PathBuf> }
```

## Source
Lines 250–269 in `crates/oxide-app/src/app/handlers/canvas/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [canvas](/crates/oxide-app/src/app/handlers/canvas/mod.md) |
| calls | [resolve_child_reference](/crates/oxide-app/src/app/project_sheets/resolve_child_reference.md) |
