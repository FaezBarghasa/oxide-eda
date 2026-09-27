---
okf_version: "0.2"
type: Function
title: compute_library_row_detail
description: F15 — When the active tab is a Library Browser AND a row is
resource: crates/oxide-app/src/app/runtime/panel_ctx.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-14T10:32:07Z"
concept_id: crates/oxide-app/src/app/runtime/panel_ctx/compute_library_row_detail_1
language: rust
---

# compute_library_row_detail

F15 — When the active tab is a Library Browser AND a row is

## Signature

```rust
fn compute_library_row_detail(&self) -> Option<crate::panels::LibraryRowDetail>
```

## Docstring

F15 — When the active tab is a Library Browser AND a row is
selected in that tab's browser state, build the
[`crate::panels::LibraryRowDetail`] the Properties panel
renders. Returns `None` for any other active tab kind, or
when the browser tab has no row selected.

## Source
Lines 11–85 in `crates/oxide-app/src/app/runtime/panel_ctx.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [panel_ctx](/crates/oxide-app/src/app/runtime/panel_ctx.md) |
| calls | [find](/crates/oxide-app/src/library/editor/symbol/context_menu/rows/find.md) |
