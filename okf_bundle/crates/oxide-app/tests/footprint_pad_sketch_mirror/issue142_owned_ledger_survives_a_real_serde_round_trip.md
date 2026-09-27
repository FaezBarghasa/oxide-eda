---
okf_version: "0.2"
type: Function
title: issue142_owned_ledger_survives_a_real_serde_round_trip
description: "The durable ledger has to survive the REAL serialiser, not just"
resource: crates/oxide-app/tests/footprint_pad_sketch_mirror.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/tests/footprint_pad_sketch_mirror/issue142_owned_ledger_survives_a_real_serde_round_trip
language: rust
---

# issue142_owned_ledger_survives_a_real_serde_round_trip

The durable ledger has to survive the REAL serialiser, not just

## Signature

```rust
fn issue142_owned_ledger_survives_a_real_serde_round_trip()
```

## Decorators

- `test`

## Docstring

The durable ledger has to survive the REAL serialiser, not just
`from_footprint`.

The whole reopen fix rests on `PadAttr::owned` round-tripping
through `Footprint`'s serde derive — the same `serde_json` path
`local_git::primitives` writes with. The other reopen tests cross
the `EditorPad`-volatility boundary but never the serde one, so
nothing pinned the field's `#[serde(default,
skip_serializing_if = "Vec::is_empty")]` attributes or its presence
in the struct at all.
[test]

## Source
Lines 342–382 in `crates/oxide-app/tests/footprint_pad_sketch_mirror.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [footprint_pad_sketch_mirror](/crates/oxide-app/tests/footprint_pad_sketch_mirror.md) |
| calls | [footprint_with_minted_pad](/crates/oxide-app/tests/footprint_pad_sketch_mirror/footprint_with_minted_pad.md) |
| calls | [mirror_delete_pad_from_sketch](/crates/oxide-app/src/library/editor/footprint/pad_to_sketch/mod/mirror_delete_pad_from_sketch.md) |
