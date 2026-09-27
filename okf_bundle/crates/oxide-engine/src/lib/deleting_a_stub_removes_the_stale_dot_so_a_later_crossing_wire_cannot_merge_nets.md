---
okf_version: "0.2"
type: Function
title: deleting_a_stub_removes_the_stale_dot_so_a_later_crossing_wire_cannot_merge_nets
description: "Issue #422's root-cause gap: the stale-dot removal pass ran only via"
resource: crates/oxide-engine/src/lib.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-21T12:59:13Z"
concept_id: crates/oxide-engine/src/lib/deleting_a_stub_removes_the_stale_dot_so_a_later_crossing_wire_cannot_merge_nets
language: rust
---

# deleting_a_stub_removes_the_stale_dot_so_a_later_crossing_wire_cannot_merge_nets

Issue #422's root-cause gap: the stale-dot removal pass ran only via

## Signature

```rust
fn deleting_a_stub_removes_the_stale_dot_so_a_later_crossing_wire_cannot_merge_nets()
```

## Decorators

- `test`

## Docstring

Issue #422's root-cause gap: the stale-dot removal pass ran only via
`reconciled_patch()` for move/rotate/mirror. `DeleteSelection` and
`PlaceWireSegment` never reconciled, so a dot minted for a genuine T
could outlive the wire that justified it, then get silently
"reactivated" by an unrelated wire merely crossing the same point —
merging two nets the user never connected (a FAB short).
[test]

## Source
Lines 898–946 in `crates/oxide-engine/src/lib.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [lib](/crates/oxide-engine/src/lib.md) |
| calls | [test_sheet](/crates/oxide-engine/src/test_support/test_sheet.md) |
| calls | [wire](/crates/oxide-engine/src/lib/wire.md) |
