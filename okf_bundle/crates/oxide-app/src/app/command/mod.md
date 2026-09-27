---
okf_version: "0.2"
type: Module
title: command
description: "Home of the Command Registry (#278). Slice 1 landed the"
resource: crates/oxide-app/src/app/command/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/command/mod
language: rust
---

# command

Home of the Command Registry (#278). Slice 1 landed the

## Docstring

Home of the Command Registry (#278). Slice 1 landed the
id→[`crate::app::Message`] bridge (`bridge.rs`). Slice 2 adds the
registry's dispatch entry point ([`Oxide::dispatch_command`]) and
its argument type ([`CommandArgs`]); enablement gating (slice 3) and
rewiring menus/palette onto it (slice 4) are still ahead.

## Relationships

| Type | Target |
|------|--------|
| related | [dispatch_command](/crates/oxide-app/src/app/command/mod/dispatch_command.md) |
| related | [dispatch_command](/crates/oxide-app/src/app/command/mod/dispatch_command.md) |
| related | [known_id_dispatches_to_the_right_message](/crates/oxide-app/src/app/command/mod/known_id_dispatches_to_the_right_message.md) |
| related | [unknown_id_is_a_no_op_and_does_not_panic](/crates/oxide-app/src/app/command/mod/unknown_id_is_a_no_op_and_does_not_panic.md) |
| related | [catalog_id_with_no_bridge_arm_is_a_no_op_and_does_not_panic](/crates/oxide-app/src/app/command/mod/catalog_id_with_no_bridge_arm_is_a_no_op_and_does_not_panic.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
