---
okf_version: "0.2"
type: Module
title: template
description: "Sheet templates — `.snxsht` format. See `OUTPUT_PLAN.md` §4."
resource: crates/oxide-output/src/template/mod.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-output"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-output/src/template/mod
language: rust
---

# template

Sheet templates — `.snxsht` format. See `OUTPUT_PLAN.md` §4.

## Docstring

Sheet templates — `.snxsht` format. See `OUTPUT_PLAN.md` §4.

A template describes how a page is framed: page size, orientation,
border margin, optional zone markers, title-block layout with
substitution-aware default text for each field. Templates are rendered
on top of the schematic content during PDF export and print preview.

17 built-in templates (ISO A0-A5 + ANSI A-E, portrait + landscape where
standard practice allows) ship with the binary — see `builtin.rs`. User
templates (`.snxsht` files on disk) are a later concern; `format.rs` is
reserved for the parser/emitter when custom templates ship.

## Relationships

| Type | Target |
|------|--------|
| related | [TemplateId](/crates/oxide-output/src/template/mod/TemplateId.md) |
| related | [default](/crates/oxide-output/src/template/mod/default.md) |
| related | [default](/crates/oxide-output/src/template/mod/default.md) |
| related | [from](/crates/oxide-output/src/template/mod/from.md) |
| related | [from](/crates/oxide-output/src/template/mod/from.md) |
| related | [Template](/crates/oxide-output/src/template/mod/Template.md) |
| related | [Frame](/crates/oxide-output/src/template/mod/Frame.md) |
| related | [default](/crates/oxide-output/src/template/mod/default.md) |
| related | [default](/crates/oxide-output/src/template/mod/default.md) |
| related | [TitleBlock](/crates/oxide-output/src/template/mod/TitleBlock.md) |
| related | [TitleBlockField](/crates/oxide-output/src/template/mod/TitleBlockField.md) |
| related | [FontStyle](/crates/oxide-output/src/template/mod/FontStyle.md) |
| related | [TemplateError](/crates/oxide-output/src/template/mod/TemplateError.md) |
| related | [thiserror](/_dependencies/cargo/thiserror.md) |
