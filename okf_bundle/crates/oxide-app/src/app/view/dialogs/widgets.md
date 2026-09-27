---
okf_version: "0.2"
type: Module
title: widgets
description: "Shared modal-chrome primitives — backdrop wrapper, draggable / detached"
resource: crates/oxide-app/src/app/view/dialogs/widgets.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/dialogs/widgets
language: rust
---

# widgets

Shared modal-chrome primitives — backdrop wrapper, draggable / detached

## Docstring

Shared modal-chrome primitives — backdrop wrapper, draggable / detached
headers, the close-X button, and the common button / section builders
every dialog family reaches for.

Extracted verbatim from `view/dialogs.rs` (ADR-0001, issue #164) as pure
code motion — no behaviour change.

## Relationships

| Type | Target |
|------|--------|
| related | [wrap_modal](/crates/oxide-app/src/app/view/dialogs/widgets/wrap_modal.md) |
| related | [draggable_header](/crates/oxide-app/src/app/view/dialogs/widgets/draggable_header.md) |
| related | [detached_header](/crates/oxide-app/src/app/view/dialogs/widgets/detached_header.md) |
| related | [close_x_button](/crates/oxide-app/src/app/view/dialogs/widgets/close_x_button.md) |
| related | [close_button](/crates/oxide-app/src/app/view/dialogs/widgets/close_button.md) |
| related | [section_header](/crates/oxide-app/src/app/view/dialogs/widgets/section_header.md) |
| related | [secondary_button](/crates/oxide-app/src/app/view/dialogs/widgets/secondary_button.md) |
| related | [primary_button](/crates/oxide-app/src/app/view/dialogs/widgets/primary_button.md) |
| related | [primary_button_themed](/crates/oxide-app/src/app/view/dialogs/widgets/primary_button_themed.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
