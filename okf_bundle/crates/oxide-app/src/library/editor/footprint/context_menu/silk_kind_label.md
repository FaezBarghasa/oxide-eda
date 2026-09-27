---
okf_version: "0.2"
type: Function
title: silk_kind_label
description: "v0.26-C — surface the silk graphic''s kind in the menu header so"
resource: crates/oxide-app/src/library/editor/footprint/context_menu.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/editor/footprint/context_menu/silk_kind_label
language: rust
---

# silk_kind_label

v0.26-C — surface the silk graphic''s kind in the menu header so

## Signature

```rust
fn silk_kind_label(kind: &oxide_library::FpGraphicKind) -> &'static str
```

## Docstring

v0.26-C — surface the silk graphic''s kind in the menu header so
the user can tell at a glance what they''re about to delete /
inspect. Mirrors Altium''s naming.

## Source
Lines 35–45 in `crates/oxide-app/src/library/editor/footprint/context_menu.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [context_menu](/crates/oxide-app/src/library/editor/footprint/context_menu.md) |
