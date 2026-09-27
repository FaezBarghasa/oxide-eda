---
okf_version: "0.2"
type: Function
title: retarget_pad_profiles
description: "`CustomPadShape::SketchProfile.source` / `PasteAperturePattern::"
resource: crates/oxide-sketch/src/split/attr_refs.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-sketch/src/split/attr_refs/retarget_pad_profiles
language: rust
---

# retarget_pad_profiles

`CustomPadShape::SketchProfile.source` / `PasteAperturePattern::

## Signature

```rust
pub(super) fn retarget_pad_profiles(sketch: &mut SketchData, ctx: &SplitCtx)
```

## Visibility

- `pub(super)`

## Docstring

`CustomPadShape::SketchProfile.source` / `PasteAperturePattern::
Custom.source` are seed lists into `trace_closed_profile`'s
adjacency walk (today only `source[0]` is read — see
`oxide-bake/src/pad.rs`). The walk discovers the WHOLE closed loop
from any edge on it, and `line_a` is still wired into that same
loop (through `start`, and through the new mid Point to `line_b`),
so re-seeding with `line_a` re-finds the identical profile. Every
occurrence is rewritten, not just a first entry, in case a future
consumer reads more of the list than `source[0]`.

## Source
Lines 43–55 in `crates/oxide-sketch/src/split/attr_refs.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [attr_refs](/crates/oxide-sketch/src/split/attr_refs.md) |
| calls | [retarget](/crates/oxide-sketch/src/split/attr_refs/retarget.md) |
| called_by | [commit_split](/crates/oxide-sketch/src/split/mod/commit_split.md) |
