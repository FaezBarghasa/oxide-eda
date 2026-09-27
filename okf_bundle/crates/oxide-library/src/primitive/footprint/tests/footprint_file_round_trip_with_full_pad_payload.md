---
okf_version: "0.2"
type: Function
title: footprint_file_round_trip_with_full_pad_payload
description: All-fields round-trip — every Pad field gets a non-default
resource: crates/oxide-library/src/primitive/footprint/tests.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/primitive/footprint/tests/footprint_file_round_trip_with_full_pad_payload
language: rust
---

# footprint_file_round_trip_with_full_pad_payload

All-fields round-trip — every Pad field gets a non-default

## Signature

```rust
fn footprint_file_round_trip_with_full_pad_payload()
```

## Decorators

- `test`

## Docstring

All-fields round-trip — every Pad field gets a non-default
value (chamfered shape, non-trivial drill, multiple layers,
solder/paste margins) so the TSV cell encoders / decoders are
exercised end-to-end.
[test]

## Source
Lines 302–338 in `crates/oxide-library/src/primitive/footprint/tests.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [tests](/crates/oxide-library/src/primitive/footprint/tests.md) |
