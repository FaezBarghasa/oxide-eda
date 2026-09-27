---
okf_version: "0.2"
type: Module
title: library
description: Library subsystem dispatcher. Routes
resource: crates/oxide-app/src/app/dispatch/library/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/library/mod
language: rust
---

# library

Library subsystem dispatcher. Routes

## Docstring

Library subsystem dispatcher. Routes
[`crate::library::LibraryMessage`] to the right side-effecting
handler.

In the DBLib model the Component view is preview-only.
Symbol/Footprint/Sim render read-only here; the standalone
`.snxsym` / `.snxfpt` / `.snxsim` document tabs own actual
editing. The dispatcher's editor handlers are scoped to the five
Component Preview tabs (Preview / Parameters / Supply / Datasheet
/ Simulation).

This module is a thin router: every arm with real logic delegates
to a `handle_*` method (or free function) living in the matching
concern module below. Only the truly trivial arms (`Task::none()`,
a single state assignment) stay inline.

## Relationships

| Type | Target |
|------|--------|
| related | [dispatch_library_message](/crates/oxide-app/src/app/dispatch/library/mod/dispatch_library_message.md) |
| related | [dispatch_library_message](/crates/oxide-app/src/app/dispatch/library/mod/dispatch_library_message.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
