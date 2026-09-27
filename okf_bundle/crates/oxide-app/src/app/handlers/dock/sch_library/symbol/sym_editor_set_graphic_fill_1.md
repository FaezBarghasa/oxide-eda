---
okf_version: "0.2"
type: Function
title: sym_editor_set_graphic_fill
description: "Set (or clear, when `color` is `None`) the placed graphic's fill"
resource: crates/oxide-app/src/app/handlers/dock/sch_library/symbol.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/dock/sch_library/symbol/sym_editor_set_graphic_fill_1
language: rust
---

# sym_editor_set_graphic_fill

Set (or clear, when `color` is `None`) the placed graphic's fill

## Signature

```rust
pub(super) fn sym_editor_set_graphic_fill(
        &mut self,
        idx: usize,
        color: Option<[u8; 4]>,
    ) -> bool
```

## Visibility

- `pub(super)`

## Docstring

Set (or clear, when `color` is `None`) the placed graphic's fill
and close the picker. Dirties + clears the canvas cache.

## Source
Lines 161–184 in `crates/oxide-app/src/app/handlers/dock/sch_library/symbol.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [symbol](/crates/oxide-app/src/app/handlers/dock/sch_library/symbol.md) |
