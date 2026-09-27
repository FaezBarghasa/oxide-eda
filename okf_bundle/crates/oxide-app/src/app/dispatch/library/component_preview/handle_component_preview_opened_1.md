---
okf_version: "0.2"
type: Function
title: handle_component_preview_opened
description: "Trace-only signal: a Component Preview tab was opened for the"
resource: crates/oxide-app/src/app/dispatch/library/component_preview.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/component_preview/handle_component_preview_opened_1
language: rust
---

# handle_component_preview_opened

Trace-only signal: a Component Preview tab was opened for the

## Signature

```rust
pub(super) fn handle_component_preview_opened(
        &mut self,
        path: std::path::PathBuf,
        table: String,
        row_id: RowId,
    ) -> Task<Message>
```

## Visibility

- `pub(super)`

## Docstring

Trace-only signal: a Component Preview tab was opened for the
given address. Fired alongside `OpenComponentRow`.

## Source
Lines 14–28 in `crates/oxide-app/src/app/dispatch/library/component_preview.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [component_preview](/crates/oxide-app/src/app/dispatch/library/component_preview.md) |
