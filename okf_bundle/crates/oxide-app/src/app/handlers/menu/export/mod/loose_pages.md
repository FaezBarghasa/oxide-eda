---
okf_version: "0.2"
type: Function
title: loose_pages
description: "Pages for a loose .snxsch: the unowned open tabs, active one first. An open"
resource: crates/oxide-app/src/app/handlers/menu/export/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/handlers/menu/export/mod/loose_pages
language: rust
---

# loose_pages

Pages for a loose .snxsch: the unowned open tabs, active one first. An open

## Signature

```rust
fn loose_pages(
    document_state: &crate::app::state::DocumentState,
    active_path: &PathBuf,
) -> Vec<SheetSnapshot>
```

## Docstring

Pages for a loose .snxsch: the unowned open tabs, active one first. An open
tab belonging to some *other* loaded project must not ride along.

## Source
Lines 180–206 in `crates/oxide-app/src/app/handlers/menu/export/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [export](/crates/oxide-app/src/app/handlers/menu/export/mod.md) |
| called_by | [build_export_scope](/crates/oxide-app/src/app/handlers/menu/export/mod/build_export_scope.md) |
