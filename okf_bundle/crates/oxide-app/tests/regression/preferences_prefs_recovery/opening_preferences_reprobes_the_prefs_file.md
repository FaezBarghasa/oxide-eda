---
okf_version: "0.2"
type: Function
title: opening_preferences_reprobes_the_prefs_file
description: Opening Preferences re-probes the file rather than trusting the boot
resource: crates/oxide-app/tests/regression/preferences_prefs_recovery.rs
tags:
  - "lang:rust"
  - "type:Function"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-09-13T09:33:26Z"
concept_id: crates/oxide-app/tests/regression/preferences_prefs_recovery/opening_preferences_reprobes_the_prefs_file
language: rust
---

# opening_preferences_reprobes_the_prefs_file

Opening Preferences re-probes the file rather than trusting the boot

## Signature

```rust
fn opening_preferences_reprobes_the_prefs_file()
```

## Decorators

- `test`

## Docstring

Opening Preferences re-probes the file rather than trusting the boot
snapshot, so a hand repair is picked up without a restart.

Both directions, because only one of them can fail. Asserting that a
stale flag is cleared also passes against a handler changed to an
unconditional `prefs_load_error = None` — which deletes the feature and
keeps the suite green. Seeding a genuinely broken file and demanding
the flag be SET, naming the parse failure, is the half with teeth.
[test]

## Source
Lines 224–257 in `crates/oxide-app/tests/regression/preferences_prefs_recovery.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [preferences_prefs_recovery](/crates/oxide-app/tests/regression/preferences_prefs_recovery.md) |
| calls | [serial](/crates/oxide-app/tests/regression/preferences_prefs_recovery/serial.md) |
