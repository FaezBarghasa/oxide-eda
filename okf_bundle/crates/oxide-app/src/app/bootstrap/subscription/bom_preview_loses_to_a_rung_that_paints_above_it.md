---
okf_version: "0.2"
type: Function
title: bom_preview_loses_to_a_rung_that_paints_above_it
description: "`bom_preview_open` paints in `collect_overlays`' earliest block"
resource: crates/oxide-app/src/app/bootstrap/subscription.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/bootstrap/subscription/bom_preview_loses_to_a_rung_that_paints_above_it
language: rust
---

# bom_preview_loses_to_a_rung_that_paints_above_it

`bom_preview_open` paints in `collect_overlays`' earliest block

## Signature

```rust
fn bom_preview_loses_to_a_rung_that_paints_above_it()
```

## Decorators

- `test`

## Docstring

`bom_preview_open` paints in `collect_overlays`' earliest block
(`:753`) with no exclusivity guarantee of its own, so anything
pushed later — Find & Replace (`:787`) included — paints over it
and must win Esc too.
[test]

## Source
Lines 1252–1264 in `crates/oxide-app/src/app/bootstrap/subscription.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [subscription](/crates/oxide-app/src/app/bootstrap/subscription.md) |
