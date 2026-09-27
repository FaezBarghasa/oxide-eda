---
okf_version: "0.2"
type: Function
title: reset_symbol_viewport
description: Reset per-symbol viewport state when the active symbol changes.
resource: crates/oxide-app/src/app/handlers/dock/sch_library/symbol.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/symbol/reset_symbol_viewport
language: rust
---

# reset_symbol_viewport

Reset per-symbol viewport state when the active symbol changes.

## Signature

```rust
impl Oxide { fn reset_symbol_viewport(editor: &mut crate::app::SymbolEditorState) }
```

## Docstring

Reset per-symbol viewport state when the active symbol changes.

Without this, a newly selected/created symbol can inherit stale
pan/zoom from the previous symbol and open at an unexpected scale.

## Source
Lines 459–463 in `crates/oxide-app/src/app/handlers/dock/sch_library/symbol.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symbol](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol.md) |
