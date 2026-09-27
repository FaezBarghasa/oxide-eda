---
okf_version: "0.2"
type: Module
title: erc
description: Electrical Rule Check (ERC) modal — per-rule severity override grid
resource: crates/oxide-app/src/app/view/dialogs/erc.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/dialogs/erc
language: rust
---

# erc

Electrical Rule Check (ERC) modal — per-rule severity override grid

## Docstring

Electrical Rule Check (ERC) modal — per-rule severity override grid
and the pin-connection matrix.

Extracted verbatim from `view/dialogs.rs` (ADR-0001, issue #164) as pure
code motion — no behaviour change. These are methods of the same
`Oxide` view impl, split across sibling files.

## Relationships

| Type | Target |
|------|--------|
| related | [view_erc_dialog](/crates/oxide-app/src/app/view/dialogs/erc/view_erc_dialog.md) |
| related | [view_erc_dialog_body](/crates/oxide-app/src/app/view/dialogs/erc/view_erc_dialog_body.md) |
| related | [view_erc_dialog_body_inner](/crates/oxide-app/src/app/view/dialogs/erc/view_erc_dialog_body_inner.md) |
| related | [view_erc_dialog](/crates/oxide-app/src/app/view/dialogs/erc/view_erc_dialog.md) |
| related | [view_erc_dialog_body](/crates/oxide-app/src/app/view/dialogs/erc/view_erc_dialog_body.md) |
| related | [view_erc_dialog_body_inner](/crates/oxide-app/src/app/view/dialogs/erc/view_erc_dialog_body_inner.md) |
| related | [severity_segmented](/crates/oxide-app/src/app/view/dialogs/erc/severity_segmented.md) |
| related | [pin_matrix_view](/crates/oxide-app/src/app/view/dialogs/erc/pin_matrix_view.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
