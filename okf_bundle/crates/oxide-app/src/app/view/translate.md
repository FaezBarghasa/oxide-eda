---
okf_version: "0.2"
type: Module
title: translate
description: Custom widget that positions its child at an absolute offset from the
resource: crates/oxide-app/src/app/view/translate.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/app/view/translate
language: rust
---

# translate

Custom widget that positions its child at an absolute offset from the

## Docstring

Custom widget that positions its child at an absolute offset from the
parent's top-left corner and allows the child to extend past the parent's
bounds (i.e. off-screen). Used by modal dialogs and floating panels so the
user can drag them to any position including partially past the viewport.

Unlike `iced::widget::Pin`, which shrinks the child's available space by
the offset amount (and would squeeze a fixed-size modal when positioned
near an edge), `Translate` passes the parent's full limits through to the
child and only translates the resulting layout node. Negative offsets are
allowed; no clipping is applied inside the widget.

## Relationships

| Type | Target |
|------|--------|
| related | [Translate](/crates/oxide-app/src/app/view/translate/Translate.md) |
| related | [new](/crates/oxide-app/src/app/view/translate/new.md) |
| related | [new](/crates/oxide-app/src/app/view/translate/new.md) |
| related | [tag](/crates/oxide-app/src/app/view/translate/tag.md) |
| related | [state](/crates/oxide-app/src/app/view/translate/state.md) |
| related | [children](/crates/oxide-app/src/app/view/translate/children.md) |
| related | [diff](/crates/oxide-app/src/app/view/translate/diff.md) |
| related | [size](/crates/oxide-app/src/app/view/translate/size.md) |
| related | [layout](/crates/oxide-app/src/app/view/translate/layout.md) |
| related | [operate](/crates/oxide-app/src/app/view/translate/operate.md) |
| related | [update](/crates/oxide-app/src/app/view/translate/update.md) |
| related | [mouse_interaction](/crates/oxide-app/src/app/view/translate/mouse_interaction.md) |
| related | [draw](/crates/oxide-app/src/app/view/translate/draw.md) |
| related | [overlay](/crates/oxide-app/src/app/view/translate/overlay.md) |
| related | [tag](/crates/oxide-app/src/app/view/translate/tag.md) |
| related | [state](/crates/oxide-app/src/app/view/translate/state.md) |
| related | [children](/crates/oxide-app/src/app/view/translate/children.md) |
| related | [diff](/crates/oxide-app/src/app/view/translate/diff.md) |
| related | [size](/crates/oxide-app/src/app/view/translate/size.md) |
| related | [layout](/crates/oxide-app/src/app/view/translate/layout.md) |
| related | [operate](/crates/oxide-app/src/app/view/translate/operate.md) |
| related | [update](/crates/oxide-app/src/app/view/translate/update.md) |
| related | [mouse_interaction](/crates/oxide-app/src/app/view/translate/mouse_interaction.md) |
| related | [draw](/crates/oxide-app/src/app/view/translate/draw.md) |
| related | [overlay](/crates/oxide-app/src/app/view/translate/overlay.md) |
| related | [from](/crates/oxide-app/src/app/view/translate/from.md) |
| related | [from](/crates/oxide-app/src/app/view/translate/from.md) |
| related | [translate](/crates/oxide-app/src/app/view/translate/translate.md) |
| related | [iced](/_dependencies/cargo/iced.md) |
