---
okf_version: "0.2"
type: Function
title: selection_summary
description: Format a one-line breakdown of the active canvas selection.
resource: crates/oxide-app/src/status_bar.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/status_bar/selection_summary
language: rust
---

# selection_summary

Format a one-line breakdown of the active canvas selection.

## Signature

```rust
fn selection_summary(selected: &[SelectedItem]) -> Option<String>
```

## Docstring

Format a one-line breakdown of the active canvas selection.
Returns `None` when the selection is empty (status bar omits the segment).

## Source
Lines 16–51 in `crates/oxide-app/src/status_bar.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [status_bar](/crates/oxide-app/src/status_bar.md) |
| called_by | [view](/crates/oxide-app/src/status_bar/view.md) |
