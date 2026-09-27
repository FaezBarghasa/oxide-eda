---
okf_version: "0.2"
type: Class
title: ExpansionMode
description: v0.21 — Altium-parity PASTE / SOLDER expansion mode picker.
resource: crates/oxide-app/src/panels/footprint_editor_properties/pad/stack_preview.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/panels/footprint_editor_properties/pad/stack_preview/ExpansionMode
language: rust
---

# ExpansionMode

v0.21 — Altium-parity PASTE / SOLDER expansion mode picker.

## Signature

```rust
pub(super) enum ExpansionMode
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq)`

## Visibility

- `pub(super)`

## Docstring

v0.21 — Altium-parity PASTE / SOLDER expansion mode picker.
`Rule` defers to the per-board design rule (Solder Mask Expansion
/ Paste Mask Expansion). `Manual` overrides with an explicit
per-pad value (consumed by the matching expansion column).
[derive(Debug, Clone, Copy, PartialEq, Eq)]

## Source
Lines 607–610 in `crates/oxide-app/src/panels/footprint_editor_properties/pad/stack_preview.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [stack_preview](/crates/oxide-app/src/panels/footprint_editor_properties/pad/stack_preview.md) |
