---
okf_version: "0.2"
type: Class
title: OpenOverlays
description: "Which overlays are open, as the Esc ladder sees them."
resource: crates/oxide-app/src/app/bootstrap/subscription.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/bootstrap/subscription/OpenOverlays
language: rust
---

# OpenOverlays

Which overlays are open, as the Esc ladder sees them.

## Signature

```rust
struct OpenOverlays
```

## Decorators

- `derive(Clone, Default)`

## Docstring

Which overlays are open, as the Esc ladder sees them.

A named struct rather than the positional tuple this used to be: with
twelve same-typed `bool`s, swapping two of them silently routed Esc to
the wrong modal, with nothing to catch it.

**Built in `update`, not in the subscription** (#535). Until then this
was baked into `Subscription::with`, which forced `Hash` and — by
choice, not by the bound — `Copy`. That made per-modal payloads
unrepresentable: the per-browser delete-confirm rung could not exist,
because its Cancel message needs an owned `PathBuf`. Resolving against
live state instead drops both, and removes a one-update staleness
window as well, since the subscription snapshot was rebuilt only
*after* `update` returned — so an Esc in the same frame as a
click-Close saw the pre-click world.

Only [`KeyContext`] stays in the hashed subscription payload now.
[derive(Clone, Default)]

## Methods

- `find_replace_open`
- `kbd_shortcuts_open`
- `first_run_tour_open`
- `prefs_open`
- `annotate_open`
- `erc_open`
- `rename_open`
- `remove_open`
- `enable_vc_open`
- `library_create_options_open`
- `passive_calculator_open`
- `annotate_reset_confirm_open`
- `app_quit_confirm_open`
- `project_close_confirm_open`
- `project_options_open`
- `grid_properties_open`
- `selection_filter_custom_open`
- `library_picker_open`
- `library_document_options_open`
- `library_updates_open`
- `library_primitive_picker_open`
- `close_library_confirm_open`
- `recovery_kind`
- `error_notice_open`
- `netlist_incomplete_prompt_open`
- `print_preview_open`
- `bom_preview_open`
- `net_color_custom_open`
- `delete_confirm`

## Source
Lines 27–74 in `crates/oxide-app/src/app/bootstrap/subscription.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [subscription](/crates/oxide-app/src/app/bootstrap/subscription.md) |
