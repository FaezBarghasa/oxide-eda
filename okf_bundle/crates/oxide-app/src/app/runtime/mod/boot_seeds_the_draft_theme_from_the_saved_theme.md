---
okf_version: "0.2"
type: Function
title: boot_seeds_the_draft_theme_from_the_saved_theme
description: "#631 — `preferences_draft_theme` was hardcoded to `Oxide` at boot"
resource: crates/oxide-app/src/app/runtime/mod.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/runtime/mod/boot_seeds_the_draft_theme_from_the_saved_theme
language: rust
---

# boot_seeds_the_draft_theme_from_the_saved_theme

#631 — `preferences_draft_theme` was hardcoded to `Oxide` at boot

## Signature

```rust
fn boot_seeds_the_draft_theme_from_the_saved_theme()
```

## Decorators

- `test`

## Docstring

#631 — `preferences_draft_theme` was hardcoded to `Oxide` at boot
rather than seeded from the saved preference. That was invisible
while nothing read it before the Preferences dialog first opened
(`seed_preferences_drafts_from_live` repaired it there). The canvas
reads it now, so a user whose saved theme is not Oxide would have
opened to the wrong canvas colours.
[test]

## Source
Lines 345–359 in `crates/oxide-app/src/app/runtime/mod.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [runtime](/crates/oxide-app/src/app/runtime/mod.md) |
