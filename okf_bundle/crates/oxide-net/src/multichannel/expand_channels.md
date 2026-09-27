---
okf_version: "0.2"
type: Function
title: expand_channels
description: Expand a template schematic sheet across N repeated channels.
resource: crates/oxide-net/src/multichannel.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T09:12:02Z"
concept_id: crates/oxide-net/src/multichannel/expand_channels
language: rust
---

# expand_channels

Expand a template schematic sheet across N repeated channels.

## Signature

```rust
impl MultiChannelEngine { pub fn expand_channels(
        template_sheet: &SchematicSheet,
        instantiation: &ChannelInstantiation,
    ) -> (Vec<SchematicSheet>, Vec<ChannelComponent>) }
```

## Visibility

- `pub`

## Docstring

Expand a template schematic sheet across N repeated channels.

## Source
Lines 34–69 in `crates/oxide-net/src/multichannel.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [multichannel](/crates/oxide-net/src/multichannel.md) |
