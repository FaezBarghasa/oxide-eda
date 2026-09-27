---
okf_version: "0.2"
type: Module
title: subscription
description: "`Oxide::subscription` — keyboard / window / tick event wiring."
resource: crates/oxide-app/src/app/bootstrap/subscription.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/bootstrap/subscription
language: rust
---

# subscription

`Oxide::subscription` — keyboard / window / tick event wiring.

## Docstring

`Oxide::subscription` — keyboard / window / tick event wiring.
Split from `app/bootstrap.rs` as pure code motion.

## Relationships

| Type | Target |
|------|--------|
| related | [OpenOverlays](/crates/oxide-app/src/app/bootstrap/subscription/OpenOverlays.md) |
| related | [RecoveryKind](/crates/oxide-app/src/app/bootstrap/subscription/RecoveryKind.md) |
| related | [escape_message](/crates/oxide-app/src/app/bootstrap/subscription/escape_message.md) |
| related | [has_blocking_modal](/crates/oxide-app/src/app/bootstrap/subscription/has_blocking_modal.md) |
| related | [rung](/crates/oxide-app/src/app/bootstrap/subscription/rung.md) |
| related | [escape_message](/crates/oxide-app/src/app/bootstrap/subscription/escape_message.md) |
| related | [has_blocking_modal](/crates/oxide-app/src/app/bootstrap/subscription/has_blocking_modal.md) |
| related | [rung](/crates/oxide-app/src/app/bootstrap/subscription/rung.md) |
| related | [modal_overlay_id](/crates/oxide-app/src/app/bootstrap/subscription/modal_overlay_id.md) |
| related | [escape_overlay_message](/crates/oxide-app/src/app/bootstrap/subscription/escape_overlay_message.md) |
| related | [detached_modal_escape_message](/crates/oxide-app/src/app/bootstrap/subscription/detached_modal_escape_message.md) |
| related | [open_overlays](/crates/oxide-app/src/app/bootstrap/subscription/open_overlays.md) |
| related | [subscription](/crates/oxide-app/src/app/bootstrap/subscription/subscription.md) |
| related | [escape_overlay_message](/crates/oxide-app/src/app/bootstrap/subscription/escape_overlay_message.md) |
| related | [detached_modal_escape_message](/crates/oxide-app/src/app/bootstrap/subscription/detached_modal_escape_message.md) |
| related | [open_overlays](/crates/oxide-app/src/app/bootstrap/subscription/open_overlays.md) |
| related | [subscription](/crates/oxide-app/src/app/bootstrap/subscription/subscription.md) |
| related | [only](/crates/oxide-app/src/app/bootstrap/subscription/only.md) |
| related | [nothing_open_falls_through_to_the_tool_reset](/crates/oxide-app/src/app/bootstrap/subscription/nothing_open_falls_through_to_the_tool_reset.md) |
| related | [a_blocking_modal_always_claims_escape_itself](/crates/oxide-app/src/app/bootstrap/subscription/a_blocking_modal_always_claims_escape_itself.md) |
| related | [every_detachment_filtered_modal_can_still_answer_its_own_escape](/crates/oxide-app/src/app/bootstrap/subscription/every_detachment_filtered_modal_can_still_answer_its_own_escape.md) |
| related | [a_detached_modal_answers_escape_with_the_same_message_its_card_would](/crates/oxide-app/src/app/bootstrap/subscription/a_detached_modal_answers_escape_with_the_same_message_its_card_would.md) |
| related | [passive_calculator_claims_escape](/crates/oxide-app/src/app/bootstrap/subscription/passive_calculator_claims_escape.md) |
| related | [every_modal_claims_escape](/crates/oxide-app/src/app/bootstrap/subscription/every_modal_claims_escape.md) |
| related | [delete_confirm_ranks_below_the_primitive_picker_and_above_the_library_picker](/crates/oxide-app/src/app/bootstrap/subscription/delete_confirm_ranks_below_the_primitive_picker_and_above_the_library_picker.md) |
| related | [the_deepest_modal_wins](/crates/oxide-app/src/app/bootstrap/subscription/the_deepest_modal_wins.md) |
| related | [annotate_reset_confirm_outranks_annotate_dialog](/crates/oxide-app/src/app/bootstrap/subscription/annotate_reset_confirm_outranks_annotate_dialog.md) |
| related | [quit_gate_outranks_a_dialog_layered_under_it](/crates/oxide-app/src/app/bootstrap/subscription/quit_gate_outranks_a_dialog_layered_under_it.md) |
| related | [blocking_modal_outranks_a_quit_gate_set_behind_it](/crates/oxide-app/src/app/bootstrap/subscription/blocking_modal_outranks_a_quit_gate_set_behind_it.md) |
| related | [alt_f4_behind_an_error_notice_card_does_not_steal_escape](/crates/oxide-app/src/app/bootstrap/subscription/alt_f4_behind_an_error_notice_card_does_not_steal_escape.md) |
| related | [blocking_modal_group_internal_order_matches_derived_position](/crates/oxide-app/src/app/bootstrap/subscription/blocking_modal_group_internal_order_matches_derived_position.md) |
| related | [erc_dialog_outranks_enable_version_control](/crates/oxide-app/src/app/bootstrap/subscription/erc_dialog_outranks_enable_version_control.md) |
| related | [grid_properties_outranks_enable_version_control](/crates/oxide-app/src/app/bootstrap/subscription/grid_properties_outranks_enable_version_control.md) |
| related | [bom_preview_loses_to_a_rung_that_paints_above_it](/crates/oxide-app/src/app/bootstrap/subscription/bom_preview_loses_to_a_rung_that_paints_above_it.md) |
| related | [library_document_options_outranks_library_picker](/crates/oxide-app/src/app/bootstrap/subscription/library_document_options_outranks_library_picker.md) |
| related | [a_dialog_pushed_after_the_quit_gate_outranks_it](/crates/oxide-app/src/app/bootstrap/subscription/a_dialog_pushed_after_the_quit_gate_outranks_it.md) |
| related | [quit_gate_outranks_bom_preview_because_it_is_not_actually_blocking](/crates/oxide-app/src/app/bootstrap/subscription/quit_gate_outranks_bom_preview_because_it_is_not_actually_blocking.md) |
| related | [every_ladder_field_claims_escape](/crates/oxide-app/src/app/bootstrap/subscription/every_ladder_field_claims_escape.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
