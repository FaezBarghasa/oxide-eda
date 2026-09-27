---
okf_version: "0.2"
type: Function
title: handle_selection_cut_requested
description: "Cut is copy + delete, but Copy (`collect_selection_clipboard`)"
resource: crates/oxide-app/src/app/handlers/clipboard_workflows.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/handlers/clipboard_workflows/handle_selection_cut_requested
language: rust
---

# handle_selection_cut_requested

Cut is copy + delete, but Copy (`collect_selection_clipboard`)

## Signature

```rust
impl Oxide { pub(crate) fn handle_selection_cut_requested(&mut self) -> Task<Message> }
```

## Visibility

- `pub(crate)`

## Docstring

Cut is copy + delete, but Copy (`collect_selection_clipboard`)
silently drops kinds it can't carry — `ChildSheet`/`SheetPin`
today. Deleting those anyway would destroy them with nothing in
the clipboard to restore on paste, so only the clipboard-carriable
subset of the selection is cut; anything else stays selected and
untouched (#341 — sheet clipboard support itself stays out of
scope; Cut just can't silently eat what Copy drops).

## Source
Lines 144–159 in `crates/oxide-app/src/app/handlers/clipboard_workflows.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [clipboard_workflows](/crates/oxide-app/src/app/handlers/clipboard_workflows.md) |
| calls | [partition_cuttable](/crates/oxide-engine/src/selection/partition_cuttable.md) |
