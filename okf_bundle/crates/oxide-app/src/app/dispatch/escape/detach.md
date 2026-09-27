---
okf_version: "0.2"
type: Function
title: detach
resource: crates/oxide-app/src/app/dispatch/escape.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/escape/detach
language: rust
---

# detach

## Signature

```rust
fn detach(app: &mut Oxide, modal: ModalId) -> iced::window::Id
```

## Source
Lines 223–225 in `crates/oxide-app/src/app/dispatch/escape.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [escape](/crates/oxide-app/src/app/dispatch/escape.md) |
| calls | [open_window](/crates/oxide-app/src/app/dispatch/escape/open_window.md) |
| called_by | [a_detached_modal_with_no_rung_still_swallows_its_own_esc](/crates/oxide-app/src/app/dispatch/escape/a_detached_modal_with_no_rung_still_swallows_its_own_esc.md) |
| called_by | [esc_in_a_detached_modal_window_addresses_that_modal_and_not_the_canvas](/crates/oxide-app/src/app/dispatch/escape/esc_in_a_detached_modal_window_addresses_that_modal_and_not_the_canvas.md) |
| called_by | [esc_in_the_main_window_still_leaves_a_detached_modal_alone](/crates/oxide-app/src/app/dispatch/escape/esc_in_the_main_window_still_leaves_a_detached_modal_alone.md) |
| called_by | [escape_source_classifies_every_window_kind](/crates/oxide-app/src/app/dispatch/escape/escape_source_classifies_every_window_kind.md) |
