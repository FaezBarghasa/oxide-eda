---
okf_version: "0.2"
type: Function
title: pdf_section_title
description: Section header strip — accent panel-bg with a 1 px border.
resource: crates/oxide-app/src/app/view/pdf_preview/settings.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/pdf_preview/settings/pdf_section_title_1
language: rust
---

# pdf_section_title

Section header strip — accent panel-bg with a 1 px border.

## Signature

```rust
fn pdf_section_title(&self, label: &'static str) -> Element<'_, Message>
```

## Docstring

Section header strip — accent panel-bg with a 1 px border.
Reused by every Settings-tab section.

## Source
Lines 34–53 in `crates/oxide-app/src/app/view/pdf_preview/settings.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [settings](/crates/oxide-app/src/app/view/pdf_preview/settings.md) |
| calls | [ti](/crates/oxide-app/src/styles/ti.md) |
