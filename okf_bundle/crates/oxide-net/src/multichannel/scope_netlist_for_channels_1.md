---
okf_version: "0.2"
type: Function
title: scope_netlist_for_channels
description: Generate channel-scoped Netlist from an existing flat netlist and channel instances.
resource: crates/oxide-net/src/multichannel.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T09:12:02Z"
concept_id: crates/oxide-net/src/multichannel/scope_netlist_for_channels_1
language: rust
---

# scope_netlist_for_channels

Generate channel-scoped Netlist from an existing flat netlist and channel instances.

## Signature

```rust
pub fn scope_netlist_for_channels(
        base_netlist: &Netlist,
        channel_count: u32,
        prefix: &str,
    ) -> Netlist
```

## Visibility

- `pub`

## Docstring

Generate channel-scoped Netlist from an existing flat netlist and channel instances.

## Source
Lines 72–122 in `crates/oxide-net/src/multichannel.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [multichannel](/crates/oxide-net/src/multichannel.md) |
| calls | [NetId](/crates/oxide-types/src/net/NetId.md) |
