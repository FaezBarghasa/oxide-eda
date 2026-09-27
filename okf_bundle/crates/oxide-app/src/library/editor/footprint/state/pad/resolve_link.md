---
okf_version: "0.2"
type: Function
title: resolve_link
description: "Pick `pad`'s centre out of the candidates sharing its number, or"
resource: crates/oxide-app/src/library/editor/footprint/state/pad.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/state/pad/resolve_link
language: rust
---

# resolve_link

Pick `pad`'s centre out of the candidates sharing its number, or

## Signature

```rust
fn resolve_link(
    pad: &EditorPad,
    candidates: &[(PosKey, oxide_sketch::id::SketchEntityId)],
    number_claims: &std::collections::HashMap<String, usize>,
    exact_claims: &std::collections::HashMap<(String, PosKey), usize>,
) -> Option<oxide_sketch::id::SketchEntityId>
```

## Docstring

Pick `pad`'s centre out of the candidates sharing its number, or
`None` when the answer is not unambiguous. See the collision section
on [`relink_pads_to_sketch`] for why `None` is the safe answer.

## Source
Lines 699–725 in `crates/oxide-app/src/library/editor/footprint/state/pad.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad](/crates/oxide-app/src/library/editor/footprint/state/pad.md) |
| calls | [pad_pos_key](/crates/oxide-app/src/library/editor/footprint/state/pad/pad_pos_key.md) |
| called_by | [relink_pads_to_sketch](/crates/oxide-app/src/library/editor/footprint/state/pad/relink_pads_to_sketch.md) |
