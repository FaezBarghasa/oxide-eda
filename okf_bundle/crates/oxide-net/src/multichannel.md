---
okf_version: "0.2"
type: Module
title: multichannel
description: Multi-Channel Schematic Hierarchy and Net Bus Expansion Engine.
resource: crates/oxide-net/src/multichannel.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T09:12:02Z"
concept_id: crates/oxide-net/src/multichannel
language: rust
---

# multichannel

Multi-Channel Schematic Hierarchy and Net Bus Expansion Engine.

## Docstring

Multi-Channel Schematic Hierarchy and Net Bus Expansion Engine.

Handles `Repeat(SheetName, StartIdx, EndIdx)` instantiation, channel-scoped net naming,
component suffix assignment (`U1_CH1`, `U1_CH2`), and multi-channel bus breakout.

## Relationships

| Type | Target |
|------|--------|
| related | [ChannelInstantiation](/crates/oxide-net/src/multichannel/ChannelInstantiation.md) |
| related | [ChannelComponent](/crates/oxide-net/src/multichannel/ChannelComponent.md) |
| related | [MultiChannelEngine](/crates/oxide-net/src/multichannel/MultiChannelEngine.md) |
| related | [expand_channels](/crates/oxide-net/src/multichannel/expand_channels.md) |
| related | [scope_netlist_for_channels](/crates/oxide-net/src/multichannel/scope_netlist_for_channels.md) |
| related | [expand_channels](/crates/oxide-net/src/multichannel/expand_channels.md) |
| related | [scope_netlist_for_channels](/crates/oxide-net/src/multichannel/scope_netlist_for_channels.md) |
| related | [test_multichannel_netlist_scoping](/crates/oxide-net/src/multichannel/test_multichannel_netlist_scoping.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
