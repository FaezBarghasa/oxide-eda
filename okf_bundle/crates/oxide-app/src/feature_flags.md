---
okf_version: "0.2"
type: Module
title: feature_flags
description: Compile-time constants for shipping incomplete subsystems dark.
resource: crates/oxide-app/src/feature_flags.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/feature_flags
language: rust
---

# feature_flags

Compile-time constants for shipping incomplete subsystems dark.

## Docstring

Compile-time constants for shipping incomplete subsystems dark.

These are plain `const bool`s, not Cargo features — the code stays
compiled (so it can't bit-rot) while its user-facing surface is held
back.

**They are not all the same kind of switch, and the difference decides
whether editing one does anything.**

- A **gate** is read at the entry points themselves, so flipping it is
the whole change. [`FOOTPRINT_EDITOR_ENABLED`] is one: four call sites
branch on it to hide the tab, the create flow and the menu items.
- A **default** is only the fallback for a preference the user can set.
Flipping it changes what a *fresh* profile starts with and nothing
else — anyone whose `prefs.json` already carries the key keeps their
value. [`PCB_GPU_RENDER_DEFAULT`] is one.

This doc used to say every const here gated its feature's entry points
and that flipping one required "no other change". That was true of the
gate and false of the default — the kind of mistake that gets made once
under pressure, reaching for a const to turn a feature off for everyone
and shipping a release where it is still on.
