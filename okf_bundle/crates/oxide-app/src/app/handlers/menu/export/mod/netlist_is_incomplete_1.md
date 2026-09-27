---
okf_version: "0.2"
type: Function
title: netlist_is_incomplete
description: Whether the derived netlist is missing connectivity rather than merely
resource: crates/oxide-app/src/app/handlers/menu/export/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:49Z"
concept_id: crates/oxide-app/src/app/handlers/menu/export/mod/netlist_is_incomplete_1
language: rust
---

# netlist_is_incomplete

Whether the derived netlist is missing connectivity rather than merely

## Signature

```rust
pub(crate) fn netlist_is_incomplete(&self) -> bool
```

## Visibility

- `pub(crate)`

## Docstring

Whether the derived netlist is missing connectivity rather than merely
oddly named.

A `MissingChild` subtree does not just leave its own nets out: nets that
should merge through that sheet's ports stay split, so the surviving
nets can carry the *wrong* names. `SheetCycle` truncates the walk with
the same effect, and a declared page with no file behind it is a whole
page of components and nets that never made it in. `SheetKeyCollision`
is a hole too, though a different one: the dropped sheet never entered
the graph at all, so its components and nets are simply absent —
nothing is stitched from the wrong file and nothing is grafted in
twice, which is what the pre-#466 filename-keyed model did and what
this variant's predecessor described. The remaining `StitchIssue`
variants (duplicate UUIDs, shared references, name collisions) describe
a complete netlist with a naming or annotation problem — loud, but not
a hole.

## Source
Lines 67–78 in `crates/oxide-app/src/app/handlers/menu/export/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [export](/crates/oxide-app/src/app/handlers/menu/export/mod.md) |
