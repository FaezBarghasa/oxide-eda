---
okf_version: "0.2"
type: Module
title: keymap
description: Keyboard shortcut profile model and runtime lookup.
resource: crates/oxide-app/src/keymap/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/keymap/mod
language: rust
---

# keymap

Keyboard shortcut profile model and runtime lookup.

## Docstring

Keyboard shortcut profile model and runtime lookup.

The shape follows the useful parts of Zed's keymap architecture while
staying native to Oxide: TOML profiles, EDA-oriented built-ins, stable
command ids, context-aware lookup, and editor-friendly conflict reporting.
