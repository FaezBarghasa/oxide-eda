---
okf_version: "0.2"
type: Function
title: handle_print_preview_rerender
description: Public alias so the dispatcher can poke a re-rasterise after
resource: crates/oxide-app/src/app/handlers/menu/export/print_preview.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/app/handlers/menu/export/print_preview/handle_print_preview_rerender
language: rust
---

# handle_print_preview_rerender

Public alias so the dispatcher can poke a re-rasterise after

## Signature

```rust
impl Oxide { pub(crate) fn handle_print_preview_rerender(&mut self) }
```

## Visibility

- `pub(crate)`

## Docstring

Public alias so the dispatcher can poke a re-rasterise after
mutating `pdf_options` directly. The private function below
was the original entry point; this just lifts visibility.

## Source
Lines 284–286 in `crates/oxide-app/src/app/handlers/menu/export/print_preview.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [print_preview](/crates/oxide-app/src/app/handlers/menu/export/print_preview.md) |
