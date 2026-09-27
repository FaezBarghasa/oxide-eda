---
okf_version: "0.2"
type: Class
title: AlignModal
description: "#370 — \"Align…\" dialog state. `None` on"
resource: crates/oxide-app/src/library/editor/footprint/state/mod.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/state/mod/AlignModal
language: rust
---

# AlignModal

#370 — "Align…" dialog state. `None` on

## Signature

```rust
pub struct AlignModal
```

## Decorators

- `derive(Debug, Clone, Default, PartialEq, Eq)`

## Visibility

- `pub`

## Docstring

#370 — "Align…" dialog state. `None` on
[`FootprintEditorState::align_modal`] means the modal is closed.

The dialog is a pure composition shell over the existing
[`AlignOp`] variants — it introduces no new geometry. The user picks
at most one horizontal op and at most one vertical op (each
`None` = "leave that axis untouched"); Confirm applies both chosen
ops under a SINGLE undo snapshot (see `updates::active_bar`). The two
axes are independent — horizontal ops touch only X, vertical ops only
Y — so applying both in sequence equals picking the two concrete
dropdown rows one at a time.
[derive(Debug, Clone, Default, PartialEq, Eq)]

## Methods

- `horizontal`
- `vertical`

## Source
Lines 89–98 in `crates/oxide-app/src/library/editor/footprint/state/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/library/editor/footprint/state/mod.md) |
