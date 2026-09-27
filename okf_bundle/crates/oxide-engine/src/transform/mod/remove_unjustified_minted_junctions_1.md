---
okf_version: "0.2"
type: Function
title: remove_unjustified_minted_junctions
description: Drop every minted junction no longer justified by a genuine wire
resource: crates/oxide-engine/src/transform/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-engine/src/transform/mod/remove_unjustified_minted_junctions_1
language: rust
---

# remove_unjustified_minted_junctions

Drop every minted junction no longer justified by a genuine wire

## Signature

```rust
fn remove_unjustified_minted_junctions(&mut self) -> bool
```

## Docstring

Drop every minted junction no longer justified by a genuine wire
meeting (see [`autoplace::wire_meeting_justifies_junction`]).
User-placed dots (`minted == false`) are never inspected here.

## Source
Lines 178–203 in `crates/oxide-engine/src/transform/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [transform](/crates/oxide-engine/src/transform/mod.md) |
| calls | [wire_meeting_justifies_junction](/crates/oxide-engine/src/transform/autoplace/wire_meeting_justifies_junction.md) |
