---
okf_version: "0.2"
type: Function
title: boot_seeds_the_grid_style_render_input_from_the_saved_pref
description: Boot has to leave the render input equal to the saved preference. A
resource: crates/oxide-app/tests/regression/render_config_grid_style.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/tests/regression/render_config_grid_style/boot_seeds_the_grid_style_render_input_from_the_saved_pref
language: rust
---

# boot_seeds_the_grid_style_render_input_from_the_saved_pref

Boot has to leave the render input equal to the saved preference. A

## Signature

```rust
fn boot_seeds_the_grid_style_render_input_from_the_saved_pref()
```

## Decorators

- `test`

## Docstring

Boot has to leave the render input equal to the saved preference. A
draft that is never seeded stays on `GridStyle::Dots` forever — a user
whose `prefs.json` says `lines` would open to dots.

#631 — the canvas no longer holds a copy; `preferences_draft_grid_style`
IS the value `view` hands to the `Program` each frame, so that is what
has to be right at boot.
[test]

## Source
Lines 44–57 in `crates/oxide-app/tests/regression/render_config_grid_style.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [render_config_grid_style](/crates/oxide-app/tests/regression/render_config_grid_style.md) |
