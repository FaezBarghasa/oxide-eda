---
okf_version: "0.2"
type: Function
title: content_library_distributors
description: Mount the live Distributor APIs panel inside the Preferences modal.
resource: crates/oxide-app/src/preferences/distributors.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:53:48Z"
concept_id: crates/oxide-app/src/preferences/distributors/content_library_distributors
language: rust
---

# content_library_distributors

Mount the live Distributor APIs panel inside the Preferences modal.

## Signature

```rust
pub(super) fn content_library_distributors(
    settings: &'a crate::library::state::DistributorSettings,
    tokens: &'a oxide_types::theme::ThemeTokens,
) -> Element<'a, PrefMsg>
```

## Type Parameters

- `'a`

## Visibility

- `pub(super)`

## Docstring

Mount the live Distributor APIs panel inside the Preferences modal.

The panel emits `LibraryMessage::Settings(_)`; we wrap every
message in `PrefMsg::LibrarySettings(_)` so the modal's outer
`Message::Preferences(PreferencesMsg::Inner(_))` map stays a single layer. The
`app/handlers/preferences.rs` handler unwraps and re-dispatches
via `Message::Library` so the canonical state writeback runs
through the same dispatcher the Tools-menu surface uses.

## Source
Lines 19–44 in `crates/oxide-app/src/preferences/distributors.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [distributors](/crates/oxide-app/src/preferences/distributors.md) |
| called_by | [build_content](/crates/oxide-app/src/preferences/mod/build_content.md) |
