---
okf_version: "0.2"
type: Function
title: pad_pos_key
resource: crates/oxide-app/src/library/editor/footprint/state/pad.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/state/pad/pad_pos_key
language: rust
---

# pad_pos_key

## Signature

```rust
fn pad_pos_key(pad: &EditorPad) -> PosKey
```

## Source
Lines 692–694 in `crates/oxide-app/src/library/editor/footprint/state/pad.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [pad](/crates/oxide-app/src/library/editor/footprint/state/pad.md) |
| calls | [pos_key](/crates/oxide-app/src/library/editor/footprint/state/pad/pos_key.md) |
| called_by | [relink_pads_to_sketch](/crates/oxide-app/src/library/editor/footprint/state/pad/relink_pads_to_sketch.md) |
| called_by | [resolve_link](/crates/oxide-app/src/library/editor/footprint/state/pad/resolve_link.md) |
