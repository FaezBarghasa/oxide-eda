---
okf_version: "0.2"
type: Function
title: strip_channel_suffix
description: "Helper function to strip channel numbers/suffixes (e.g. \"R1_CH2\" -> \"R1\", \"U1_3\" -> \"U1\", \"C1\" -> \"C1\")."
resource: crates/oxide-engine/src/room.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-engine"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T12:13:46Z"
concept_id: crates/oxide-engine/src/room/strip_channel_suffix
language: rust
---

# strip_channel_suffix

Helper function to strip channel numbers/suffixes (e.g. "R1_CH2" -> "R1", "U1_3" -> "U1", "C1" -> "C1").

## Signature

```rust
fn strip_channel_suffix(reference: &str) -> String
```

## Docstring

Helper function to strip channel numbers/suffixes (e.g. "R1_CH2" -> "R1", "U1_3" -> "U1", "C1" -> "C1").

## Source
Lines 261–267 in `crates/oxide-engine/src/room.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [room](/crates/oxide-engine/src/room.md) |
| called_by | [copy_room_format](/crates/oxide-engine/src/room/copy_room_format.md) |
