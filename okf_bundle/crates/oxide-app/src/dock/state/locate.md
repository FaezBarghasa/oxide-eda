---
okf_version: "0.2"
type: Function
title: locate
description: "Where `kind` currently lives, searching every region in display"
resource: crates/oxide-app/src/dock/state.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/dock/state/locate
language: rust
---

# locate

Where `kind` currently lives, searching every region in display

## Signature

```rust
impl DockArea { pub fn locate(&self, kind: PanelKind) -> Option<PanelSite> }
```

## Visibility

- `pub`

## Docstring

Where `kind` currently lives, searching every region in display
order and then the floating list. `None` when it is not in the
dock at all — note that it may still own a detached OS window,
which only `Oxide::show_panel` can see.

## Source
Lines 43–58 in `crates/oxide-app/src/dock/state.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [state](/crates/oxide-app/src/dock/state.md) |
