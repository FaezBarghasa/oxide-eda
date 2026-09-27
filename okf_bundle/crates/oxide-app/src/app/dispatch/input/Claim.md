---
okf_version: "0.2"
type: Class
title: Claim
description: "One consumer's answer for one event."
resource: crates/oxide-app/src/app/dispatch/input.rs
tags:
  - "lang:rust"
  - "type:Class"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:55:33Z"
concept_id: crates/oxide-app/src/app/dispatch/input/Claim
language: rust
---

# Claim

One consumer's answer for one event.

## Signature

```rust
enum Claim
```

## Decorators

- `derive(Debug)`

## Docstring

One consumer's answer for one event.

[`Claim::Swallow`] is the point of this type. Exclusivity used to be
spelled `Message::Noop`, which simultaneously means "key released",
"stroke the keymap cannot express" and "the palette ate it" — so no
test could assert that an event was deliberately swallowed. These are
now three distinct answers.

Exclusivity is a per-event answer rather than a per-consumer flag on
purpose: the palette is exclusive-*with-leaks* (Esc, the arrows and
Ctrl+Shift+P leave it as [`Claim::Consume`]) and the recorder claims
`ModifiersChanged` as well as key presses. A boolean would
misrepresent both.
[derive(Debug)]

## Source
Lines 126–133 in `crates/oxide-app/src/app/dispatch/input.rs`

## Relationships

| Type | Target |
|------|--------|
| related | [input](/crates/oxide-app/src/app/dispatch/input.md) |
