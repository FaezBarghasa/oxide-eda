---
okf_version: "0.2"
type: Function
title: try_save_dirty_path
description: Attempt to save a single dirty path during a Save-All flow
resource: crates/oxide-app/src/app/handlers/dock/project_navigation/close_project.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/project_navigation/close_project/try_save_dirty_path
language: rust
---

# try_save_dirty_path

Attempt to save a single dirty path during a Save-All flow

## Signature

```rust
impl Oxide { fn try_save_dirty_path(&mut self, path: &std::path::Path) -> anyhow::Result<()> }
```

## Docstring

Attempt to save a single dirty path during a Save-All flow
(project close or app exit), routing by the kind of document the
path backs:

* a live schematic **engine** → `engine.save()`;
* an open **symbol/footprint primitive editor** → `save_primitive_tab_at`;
* the project's own **`.snxprj`** → `save_project_at_path`.

Both Save-All loops previously handled *only* engines, so a dirty
`.snxsym`/`.snxfpt` draft (e.g. a freshly-added symbol library
like `SymbolLibrary5.snxsym`) or a dirty `.snxprj` always fell
through to the failure list and blocked the close with a bogus
"Could not save …" — the file was never actually attempted (#104).

On success the path's dirty markers are cleared: the primitive
and project savers clear their own; the engine leg clears them
here. Returns the failure reason so the caller can tell the user
WHY a file couldn't be saved instead of only naming it.

## Source
Lines 153–191 in `crates/oxide-app/src/app/handlers/dock/project_navigation/close_project.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [close_project](/crates/oxide-app/src/app/handlers/dock/project_navigation/close_project.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
