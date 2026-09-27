---
okf_version: "0.2"
type: Function
title: enable_length_tuning
description: Enable interactive length tuning with a target length and optional package delay.
resource: crates/oxide-router/src/interactive/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-router"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-22T13:13:12Z"
concept_id: crates/oxide-router/src/interactive/mod/enable_length_tuning_1
language: rust
---

# enable_length_tuning

Enable interactive length tuning with a target length and optional package delay.

## Signature

```rust
pub fn enable_length_tuning(&mut self, target_length_microns: Microns, package_delay_microns: Microns)
```

## Visibility

- `pub`

## Docstring

Enable interactive length tuning with a target length and optional package delay.

## Source
Lines 76–83 in `crates/oxide-router/src/interactive/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [interactive](/crates/oxide-router/src/interactive/mod.md) |
