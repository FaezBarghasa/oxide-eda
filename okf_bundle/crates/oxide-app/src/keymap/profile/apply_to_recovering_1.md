---
okf_version: "0.2"
type: Function
title: apply_to_recovering
description: "Apply the config, treating an unresolvable `active_profile` as"
resource: crates/oxide-app/src/keymap/profile.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/keymap/profile/apply_to_recovering_1
language: rust
---

# apply_to_recovering

Apply the config, treating an unresolvable `active_profile` as

## Signature

```rust
fn apply_to_recovering(
        self,
        set: &mut ShortcutProfileSet,
    ) -> Result<Option<String>, ProfileLoadError>
```

## Docstring

Apply the config, treating an unresolvable `active_profile` as
recoverable rather than fatal (#603).

Order is load-bearing and already was: every custom profile is
inserted before `set_active_profile` runs, so by the time the
active pointer can fail the profiles are all in `set`. That is
what makes recovery possible at all — the one thing that failed
is the pointer, and the built-in default is a fine substitute
for it.

Returns the id that could not be honoured, or `None` when the
config applied in full.

## Source
Lines 678–692 in `crates/oxide-app/src/keymap/profile.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [profile](/crates/oxide-app/src/keymap/profile.md) |
