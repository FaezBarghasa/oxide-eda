---
okf_version: "0.2"
type: Function
title: handle_library_refresh_all_pricing
description: "Library node right-click → \"Refresh All Pricing\". Stage 18 stub."
resource: crates/oxide-app/src/app/dispatch/library/browser/grid.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/browser/grid/handle_library_refresh_all_pricing_1
language: rust
---

# handle_library_refresh_all_pricing

Library node right-click → "Refresh All Pricing". Stage 18 stub.

## Signature

```rust
pub(in crate::app::dispatch::library) fn handle_library_refresh_all_pricing(
        &mut self,
        library_path: std::path::PathBuf,
    ) -> Task<Message>
```

## Visibility

- `pub(in crate::app::dispatch::library)`

## Docstring

Library node right-click → "Refresh All Pricing". Stage 18 stub.

## Source
Lines 167–183 in `crates/oxide-app/src/app/dispatch/library/browser/grid.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [grid](/crates/oxide-app/src/app/dispatch/library/browser/grid.md) |
