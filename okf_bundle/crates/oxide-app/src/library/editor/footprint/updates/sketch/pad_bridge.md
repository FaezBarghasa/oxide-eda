---
okf_version: "0.2"
type: Module
title: pad_bridge
description: Footprint sketch updates — sketch↔pad bridge (roles / profile / corner radius) concern.
resource: crates/oxide-app/src/library/editor/footprint/updates/sketch/pad_bridge.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/editor/footprint/updates/sketch/pad_bridge
language: rust
---

# pad_bridge

Footprint sketch updates — sketch↔pad bridge (roles / profile / corner radius) concern.

## Docstring

Footprint sketch updates — sketch↔pad bridge (roles / profile / corner radius) concern.

Carved out of the monolithic `sketch::apply` (ADR-0001 D1/D2). `apply`
is a thin router; each variant delegates to one named per-action fn
below (object→action, ADR-0001 D2).

## Relationships

| Type | Target |
|------|--------|
| related | [apply](/crates/oxide-app/src/library/editor/footprint/updates/sketch/pad_bridge/apply.md) |
| related | [set_role](/crates/oxide-app/src/library/editor/footprint/updates/sketch/pad_bridge/set_role.md) |
| related | [make_pad_from_profile](/crates/oxide-app/src/library/editor/footprint/updates/sketch/pad_bridge/make_pad_from_profile.md) |
| related | [unlink_corner_radius](/crates/oxide-app/src/library/editor/footprint/updates/sketch/pad_bridge/unlink_corner_radius.md) |
