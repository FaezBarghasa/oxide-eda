---
okf_version: "0.2"
type: Module
title: param
description: "Generic parameter map shared by primitives, components, and templates."
resource: crates/oxide-library/src/param.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-library"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-library/src/param
language: rust
---

# param

Generic parameter map shared by primitives, components, and templates.

## Docstring

Generic parameter map shared by primitives, components, and templates.

`ParamMap = BTreeMap<String, ParamValue>` keeps keys sorted so JSON
serialisation is deterministic — important for content-hashing and diffing.

## Relationships

| Type | Target |
|------|--------|
| related | [ParamValue](/crates/oxide-library/src/param/ParamValue.md) |
| related | [display](/crates/oxide-library/src/param/display.md) |
| related | [display](/crates/oxide-library/src/param/display.md) |
| related | [param_value_round_trip_each_variant](/crates/oxide-library/src/param/param_value_round_trip_each_variant.md) |
| related | [param_value_display_each_variant](/crates/oxide-library/src/param/param_value_display_each_variant.md) |
| related | [serde](/_dependencies/cargo/serde.md) |
