---
okf_version: "0.2"
type: Function
title: default_for
description: "The region `kind` opens into when it is not already on screen."
resource: crates/oxide-app/src/dock/placement.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T07:28:21Z"
concept_id: crates/oxide-app/src/dock/placement/default_for_1
language: rust
---

# default_for

The region `kind` opens into when it is not already on screen.

## Signature

```rust
pub fn default_for(kind: PanelKind) -> Self
```

## Visibility

- `pub`

## Docstring

The region `kind` opens into when it is not already on screen.

Every "open panel X" gesture routes through here — the View
menu, the status-bar panel list, an ERC run surfacing its
results, TAB during placement surfacing Properties, re-docking
a floating panel, closing a detached panel window. Before #641
each of those picked its own region, so the same kind could be
docked in all three at once: Signal booted Left, the View menu
added a second one Bottom, and the panel list a third Right.

The values match the first-run layout seeded in
`app/bootstrap/new.rs` for the kinds it seeds. Kinds reachable
only from the panel list default to `Right`, which is where
that list used to send everything.

## Source
Lines 21–53 in `crates/oxide-app/src/dock/placement.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [placement](/crates/oxide-app/src/dock/placement.md) |
