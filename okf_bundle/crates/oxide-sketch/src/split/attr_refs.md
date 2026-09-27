---
okf_version: "0.2"
type: Module
title: attr_refs
description: "Rewrites the retired Line's id wherever it appears OUTSIDE"
resource: crates/oxide-sketch/src/split/attr_refs.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-sketch"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-sketch/src/split/attr_refs
language: rust
---

# attr_refs

Rewrites the retired Line's id wherever it appears OUTSIDE

## Docstring

Rewrites the retired Line's id wherever it appears OUTSIDE
`entities` / `constraints` — `SketchData::arrays` and the pad
Custom-shape / Custom-paste-aperture profile-seed lists nested
inside `Entity::pad`. See [`super::split_line`]'s doc comment for
the per-collection carry-over rule these implement.

## Relationships

| Type | Target |
|------|--------|
| related | [retarget_arrays](/crates/oxide-sketch/src/split/attr_refs/retarget_arrays.md) |
| related | [retarget_pad_profiles](/crates/oxide-sketch/src/split/attr_refs/retarget_pad_profiles.md) |
| related | [retarget](/crates/oxide-sketch/src/split/attr_refs/retarget.md) |
