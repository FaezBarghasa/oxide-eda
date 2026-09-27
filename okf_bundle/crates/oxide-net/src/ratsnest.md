---
okf_version: "0.2"
type: Module
title: ratsnest
description: "Live Ratsnest & Unrouted Connectivity Engine (Phase 1.6)."
resource: crates/oxide-net/src/ratsnest.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-net"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T20:14:41Z"
concept_id: crates/oxide-net/src/ratsnest
language: rust
---

# ratsnest

Live Ratsnest & Unrouted Connectivity Engine (Phase 1.6).

## Docstring

Live Ratsnest & Unrouted Connectivity Engine (Phase 1.6).

Calculates minimal spanning trees (MST) of unrouted connections for all
pads sharing identical net IDs on a PCB layout, accounting for routed
copper segments and vias.

## Relationships

| Type | Target |
|------|--------|
| related | [RatsnestLine](/crates/oxide-net/src/ratsnest/RatsnestLine.md) |
| related | [RatsnestEngine](/crates/oxide-net/src/ratsnest/RatsnestEngine.md) |
| related | [compute_ratsnest](/crates/oxide-net/src/ratsnest/compute_ratsnest.md) |
| related | [compute_mst](/crates/oxide-net/src/ratsnest/compute_mst.md) |
| related | [compute_ratsnest](/crates/oxide-net/src/ratsnest/compute_ratsnest.md) |
| related | [compute_mst](/crates/oxide-net/src/ratsnest/compute_mst.md) |
| related | [test_compute_ratsnest_mst](/crates/oxide-net/src/ratsnest/test_compute_ratsnest_mst.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
