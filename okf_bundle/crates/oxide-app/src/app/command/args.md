---
okf_version: "0.2"
type: Module
title: args
description: "[`CommandArgs`] — the Command Registry's argument bag (#278 slice 2)."
resource: crates/oxide-app/src/app/command/args.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/command/args
language: rust
---

# args

[`CommandArgs`] — the Command Registry's argument bag (#278 slice 2).

## Docstring

[`CommandArgs`] — the Command Registry's argument bag (#278 slice 2).

All 134 catalog commands are nullary today, so this type has nothing
to carry yet. It exists only so [`super::Oxide::dispatch_command`]'s
signature will not have to change once a consumer (a future CLI is
the expected one) needs to pass data through it. Do not add an
arg-consuming command against this slice — [`super::bridge::core_to_message`]
does not read `CommandArgs` at all yet.

## Relationships

| Type | Target |
|------|--------|
| related | [CommandArgs](/crates/oxide-app/src/app/command/args/CommandArgs.md) |
| related | [none](/crates/oxide-app/src/app/command/args/none.md) |
| related | [none](/crates/oxide-app/src/app/command/args/none.md) |
