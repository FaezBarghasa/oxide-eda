---
okf_version: "0.2"
type: Function
title: handle_browser_refresh_pricing
description: "Right-click on a Library Browser row → \"Refresh Pricing\"."
resource: crates/oxide-app/src/app/dispatch/library/browser/grid.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/browser/grid/handle_browser_refresh_pricing
language: rust
---

# handle_browser_refresh_pricing

Right-click on a Library Browser row → "Refresh Pricing".

## Signature

```rust
impl Oxide { pub(in crate::app::dispatch::library) fn handle_browser_refresh_pricing(
        &mut self,
        library_path: std::path::PathBuf,
        table: String,
        row_id: RowId,
    ) -> Task<Message> }
```

## Visibility

- `pub(in crate::app::dispatch::library)`

## Docstring

Right-click on a Library Browser row → "Refresh Pricing".
Stage 18 stub.

## Source
Lines 146–164 in `crates/oxide-app/src/app/dispatch/library/browser/grid.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [grid](/crates/oxide-app/src/app/dispatch/library/browser/grid.md) |
