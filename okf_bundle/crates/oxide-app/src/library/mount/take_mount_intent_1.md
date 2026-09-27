---
okf_version: "0.2"
type: Function
title: take_mount_intent
description: Take the recorded intent for a finished mount.
resource: crates/oxide-app/src/library/mount.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/library/mount/take_mount_intent_1
language: rust
---

# take_mount_intent

Take the recorded intent for a finished mount.

## Signature

```rust
pub fn take_mount_intent(&mut self, path: &Path) -> Option<MountIntent>
```

## Visibility

- `pub`

## Docstring

Take the recorded intent for a finished mount.

`None` means the request was cancelled while the preparation was
in flight — `close_library` removes the entry, so the map doubles
as the cancellation tombstone and a completion whose path is gone
discards its payload instead of resurrecting a closed library.

The intent is read **from the map**, never from a value captured
at spawn time. Capturing it would lose the `Silent` →
`OpenBrowserTab` upgrade, which is the subtle half of this design.

## Source
Lines 239–241 in `crates/oxide-app/src/library/mount.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [mount](/crates/oxide-app/src/library/mount.md) |
| calls | [remove](/crates/oxide-app/src/library/component_preview/updates/parameters/remove.md) |
