---
okf_version: "0.2"
type: Function
title: has_blocking_modal
description: Whether a blocking modal owns the overlay stack — i.e. whether
resource: crates/oxide-app/src/app/bootstrap/subscription.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/bootstrap/subscription/has_blocking_modal
language: rust
---

# has_blocking_modal

Whether a blocking modal owns the overlay stack — i.e. whether

## Signature

```rust
impl OpenOverlays { fn has_blocking_modal(&self) -> bool }
```

## Docstring

Whether a blocking modal owns the overlay stack — i.e. whether
everything painted after the pre-blocking block is suppressed.

Mirrors `Oxide::has_blocking_modal`
(`app/view/overlays/bars.rs`). The two agreed on three of their
four terms until #547: `print_preview_open` was already false here
once Print Preview had been detached into its own OS window, while
the painter's predicate ignored detachment and kept suppressing —
so the main window painted ZERO overlays behind a detached
preview while Esc still resolved against the ones it wasn't
painting. The painter now carries the same filter and the terms
match one for one.

## Source
Lines 140–145 in `crates/oxide-app/src/app/bootstrap/subscription.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [subscription](/crates/oxide-app/src/app/bootstrap/subscription.md) |
