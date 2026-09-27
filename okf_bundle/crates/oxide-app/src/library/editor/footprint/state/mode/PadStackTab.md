---
okf_version: "0.2"
type: Class
title: PadStackTab
description: "v0.20 — Pad Stack section's tab strip. Matches Altium's three tabs"
resource: crates/oxide-app/src/library/editor/footprint/state/mode.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/library/editor/footprint/state/mode/PadStackTab
language: rust
---

# PadStackTab

v0.20 — Pad Stack section's tab strip. Matches Altium's three tabs

## Signature

```rust
pub enum PadStackTab
```

## Decorators

- `derive(Debug, Clone, Copy, PartialEq, Eq, Default)`

## Visibility

- `pub`

## Docstring

v0.20 — Pad Stack section's tab strip. Matches Altium's three tabs
verbatim:
- `Simple`: one row per stack family (COPPER / HOLE / PASTE / SOLDER).
- `TopMiddleBottom`: COPPER splits into Top / Middle / Bottom rows.
- `FullStack`: enumerates the pad's `layers` list verbatim.
[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]

## Source
Lines 76–81 in `crates/oxide-app/src/library/editor/footprint/state/mode.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mode](/crates/oxide-app/src/library/editor/footprint/state/mode.md) |
