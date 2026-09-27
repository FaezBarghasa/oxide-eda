---
okf_version: "0.2"
type: Function
title: preferences_draft_differs
description: True when any Preferences draft differs from its saved live value.
resource: crates/oxide-app/src/app/state/ui.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:25Z"
concept_id: crates/oxide-app/src/app/state/ui/preferences_draft_differs_1
language: rust
---

# preferences_draft_differs

True when any Preferences draft differs from its saved live value.

## Signature

```rust
pub fn preferences_draft_differs(&self) -> bool
```

## Visibility

- `pub`

## Docstring

True when any Preferences draft differs from its saved live value.
Single source of truth so every live-preview `PrefMsg::Draft*` handler
stays consistent as new drafts are added (the old per-handler inline
chains had drifted to different term sets). Covers ALL draft state —
the 10 appearance drafts, the component-class table and the keymap
working copy — so an appearance recompute can't report "clean" while
a pending rebind or class edit would be lost on close.

The three Symbol Editor terms joined in #629. Until then the claim
above was false: they wrote themselves to `prefs.json` on change and
were deliberately left out, so the predicate was consistent with
their behaviour but not with the footer the user was looking at.

## Source
Lines 399–418 in `crates/oxide-app/src/app/state/ui.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [ui](/crates/oxide-app/src/app/state/ui.md) |
