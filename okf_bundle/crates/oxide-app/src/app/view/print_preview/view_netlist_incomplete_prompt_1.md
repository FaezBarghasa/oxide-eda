---
okf_version: "0.2"
type: Function
title: view_netlist_incomplete_prompt
description: "#431 — netlist-incomplete \"Export anyway (incomplete)?\" prompt."
resource: crates/oxide-app/src/app/view/print_preview.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/view/print_preview/view_netlist_incomplete_prompt_1
language: rust
---

# view_netlist_incomplete_prompt

#431 — netlist-incomplete "Export anyway (incomplete)?" prompt.

## Signature

```rust
pub(super) fn view_netlist_incomplete_prompt(&self) -> Element<'_, Message>
```

## Visibility

- `pub(super)`

## Docstring

#431 — netlist-incomplete "Export anyway (incomplete)?" prompt.

Same card idiom as [`Self::view_error_notice`] (theme-token panel /
text / border; the sibling modal's severity glyph), but it leads with
the refusal explanation — unchanged severity, this is a fab deliverable
— and offers TWO actions: write the partial `.net` anyway (with the
omission recorded in its header comment) or cancel and write nothing.
The refusal stays the default; "Export anyway" is an explicit choice.

## Source
Lines 88–197 in `crates/oxide-app/src/app/view/print_preview.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [print_preview](/crates/oxide-app/src/app/view/print_preview.md) |
| calls | [ti](/crates/oxide-app/src/styles/ti.md) |
