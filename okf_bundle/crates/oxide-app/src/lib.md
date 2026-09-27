---
okf_version: "0.2"
type: Module
title: lib
description: Oxide EDA — library face of the application binary.
resource: crates/oxide-app/src/lib.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/lib
language: rust
---

# lib

Oxide EDA — library face of the application binary.

## Docstring

Oxide EDA — library face of the application binary.

`main.rs` is a thin wrapper that calls into this crate; the real
contents live here so that integration tests in `tests/` can
`use oxide_app::*` and exercise dispatchers without spinning up
the iced runtime.

All modules are `pub` for test access. The published surface is
deliberately wide because tests need to reach into engine state,
dispatch handlers, and inspect dirty bits / panel context — there's
no narrow public API to design here, the test harness IS the audit.
