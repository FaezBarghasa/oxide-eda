---
okf_version: "0.2"
type: Function
title: rung
description: "The Cancel/Close message `id` claims for Esc, or `None` when that"
resource: crates/oxide-app/src/app/bootstrap/subscription.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/bootstrap/subscription/rung
language: rust
---

# rung

The Cancel/Close message `id` claims for Esc, or `None` when that

## Signature

```rust
impl OpenOverlays { fn rung(&self, id: OverlayId) -> Option<Message> }
```

## Docstring

The Cancel/Close message `id` claims for Esc, or `None` when that
overlay is closed — or has no rung by design.

Arms are listed in `PAINT_ORDER` sequence so a reviewer can read
the array and this match side by side. The match is exhaustive,
which is what makes "a new overlay silently has no Esc" a compile
error instead of a bug report.

## Source
Lines 154–336 in `crates/oxide-app/src/app/bootstrap/subscription.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [subscription](/crates/oxide-app/src/app/bootstrap/subscription.md) |
| calls | [FindReplaceMsg](/crates/oxide-app/src/find_replace/FindReplaceMsg.md) |
