---
okf_version: "0.2"
type: Module
title: footprint
description: "Footprint canvas editor messages — the `Footprint*` family split out of"
resource: crates/oxide-app/src/library/messages/footprint.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/library/messages/footprint
language: rust
---

# footprint

Footprint canvas editor messages — the `Footprint*` family split out of

## Docstring

Footprint canvas editor messages — the `Footprint*` family split out of
the former flat `PrimitiveEditorMsg` (ADR-0001 D3). Reached through
[`super::PrimitiveEdit::Footprint`]; matched by `apply_footprint_primitive_edit`.

## Relationships

| Type | Target |
|------|--------|
| related | [FootprintEditorMsg](/crates/oxide-app/src/library/messages/footprint/FootprintEditorMsg.md) |
