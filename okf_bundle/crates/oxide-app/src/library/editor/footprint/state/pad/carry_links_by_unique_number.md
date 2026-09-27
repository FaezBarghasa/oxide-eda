---
okf_version: "0.2"
type: Function
title: carry_links_by_unique_number
description: Carry the three session-volatile link fields from the pre-refresh
resource: crates/oxide-app/src/library/editor/footprint/state/pad.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/state/pad/carry_links_by_unique_number
language: rust
---

# carry_links_by_unique_number

Carry the three session-volatile link fields from the pre-refresh

## Signature

```rust
pub(super) fn carry_links_by_unique_number(old: &[EditorPad], new_pads: &mut [EditorPad])
```

## Visibility

- `pub(super)`

## Docstring

Carry the three session-volatile link fields from the pre-refresh
pad list onto the freshly-rebuilt one, matching by pad number.

Only where the number identifies exactly ONE pad on each side.
Numbers are not unique in oxide, and a last-wins number map hands
several pads the same `sketch_entity_id` — after which a Pads-mode
delete of one pad runs the delete mirror over another's geometry
and that pad's copper silently disappears from the bake. An
ambiguous number is left unlinked here and picked up by
[`relink_pads_to_sketch`], which disambiguates by exact position
and refuses if even that ties.

## Source
Lines 561–600 in `crates/oxide-app/src/library/editor/footprint/state/pad.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad](/crates/oxide-app/src/library/editor/footprint/state/pad.md) |
| calls | [flatten](/crates/oxide-app/src/library/editor/symbol/context_menu/mod/flatten.md) |
| called_by | [refresh_pads_from_primitive](/crates/oxide-app/src/library/editor/footprint/state/mod/refresh_pads_from_primitive.md) |
