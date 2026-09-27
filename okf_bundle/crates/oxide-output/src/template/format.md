---
okf_version: "0.2"
type: Module
title: format
description: "`.snxsht` user-template parser/emitter — placeholder."
resource: crates/oxide-output/src/template/format.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-output/src/template/format
language: rust
---

# format

`.snxsht` user-template parser/emitter — placeholder.

## Docstring

`.snxsht` user-template parser/emitter — placeholder.

The legacy implementation was a Standard-style S-expression parser
built on top of `standard-parser::sexpr`. As part of the issue #62
Apache-clean cutover that codepath was removed; user-defined
templates will return when `.snxsht` is reimplemented on top of
Oxide's native TOML-based format.

The 17 built-in templates (`builtin.rs`) cover every shipping
template, so the loss is functionally the lack of *user-authored*
sheet templates. `parse_template` and `emit_template` stay as
exported symbols so any code paths referencing them keep
compiling; both currently surface `SnxshtError::NotImplemented`.

TODO(issue#62): port the template format to TOML to match
`.snxsch`/`.snxpcb`/`.snxsym` once those wire formats stabilise.

## Relationships

| Type | Target |
|------|--------|
| related | [SnxshtError](/crates/oxide-output/src/template/format/SnxshtError.md) |
| related | [parse_template](/crates/oxide-output/src/template/format/parse_template.md) |
| related | [emit_template](/crates/oxide-output/src/template/format/emit_template.md) |
