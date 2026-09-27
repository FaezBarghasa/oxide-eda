---
okf_version: "0.2"
type: Module
title: dialogs
description: Shared modal-chrome constants (single source of truth) plus re-exports
resource: crates/oxide-app/src/app/view/dialogs/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/app/view/dialogs/mod
language: rust
---

# dialogs

Shared modal-chrome constants (single source of truth) plus re-exports

## Docstring

Shared modal-chrome constants (single source of truth) plus re-exports
of the shared modal primitives, kept at this path so the existing
`crate::app::view::dialogs::…` imports across the crate keep resolving.

The dialog builders that used to live here were split into the child
modules of this `dialogs` folder (ADR-0001, issue #164) as pure code
motion.

## Relationships

| Type | Target |
|------|--------|
| related | [iced](/_dependencies/cargo/iced.md) |
