---
okf_version: "0.2"
type: Module
title: layer
description: Oxide-native PCB layer abstraction.
resource: crates/oxide-types/src/layer.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-types"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-types/src/layer
language: rust
---

# layer

Oxide-native PCB layer abstraction.

## Docstring

Oxide-native PCB layer abstraction.

Variants are **semantic** — they describe a layer's purpose
(top copper, bottom silkscreen, courtyard, etc.), not its bit
position in any particular EDA tool's internal layer set.
The previous version of this module exposed `LayerId(u8)` plus
pre-Standard-7 numeric constants (`F_CU = 0`, `B_CU = 31`, …) that
mirrored Standard's `PCB_LAYER_ID` numbering; those have been
removed as part of the issue #62 Apache-clean remediation.
Concrete `u8` IDs for any future foreign-format I/O are produced
by the `oxide-standard-import` companion crate's translation layer
and do not live in this Apache codebase.

## Relationships

| Type | Target |
|------|--------|
| related | [OxideLayer](/crates/oxide-types/src/layer/OxideLayer.md) |
| related | [LayerKind](/crates/oxide-types/src/layer/LayerKind.md) |
| related | [kind](/crates/oxide-types/src/layer/kind.md) |
| related | [altium_label](/crates/oxide-types/src/layer/altium_label.md) |
| related | [all](/crates/oxide-types/src/layer/all.md) |
| related | [kind](/crates/oxide-types/src/layer/kind.md) |
| related | [altium_label](/crates/oxide-types/src/layer/altium_label.md) |
| related | [all](/crates/oxide-types/src/layer/all.md) |
| related | [altium_labels_match_reference](/crates/oxide-types/src/layer/altium_labels_match_reference.md) |
| related | [kinds_partition_correctly](/crates/oxide-types/src/layer/kinds_partition_correctly.md) |
| related | [round_trip_json](/crates/oxide-types/src/layer/round_trip_json.md) |
| related | [all_iteration_yields_canonical_set](/crates/oxide-types/src/layer/all_iteration_yields_canonical_set.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
