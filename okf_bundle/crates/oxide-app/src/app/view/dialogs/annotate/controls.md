---
okf_version: "0.2"
type: Module
title: controls
description: Small read-only form controls used by the Annotate modal — the inline
resource: crates/oxide-app/src/app/view/dialogs/annotate/controls.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/app/view/dialogs/annotate/controls
language: rust
---

# controls

Small read-only form controls used by the Annotate modal — the inline

## Docstring

Small read-only form controls used by the Annotate modal — the inline
checkbox pip, the order-of-processing radio pill, the 2×2 order-preview
legend, and the shared bordered-container style.

Moved verbatim out of `dialogs/annotate.rs` (ADR-0001, issue #164) as
pure code motion — no behaviour change. These are free helper fns local
to the annotate concern; kept `pub(super)` so only `annotate/mod.rs`
reaches them.

## Relationships

| Type | Target |
|------|--------|
| related | [check_pip](/crates/oxide-app/src/app/view/dialogs/annotate/controls/check_pip.md) |
| related | [order_preview](/crates/oxide-app/src/app/view/dialogs/annotate/controls/order_preview.md) |
| related | [bordered_style](/crates/oxide-app/src/app/view/dialogs/annotate/controls/bordered_style.md) |
| related | [order_radio](/crates/oxide-app/src/app/view/dialogs/annotate/controls/order_radio.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
