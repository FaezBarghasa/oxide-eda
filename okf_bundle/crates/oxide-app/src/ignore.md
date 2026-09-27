---
okf_version: "0.2"
type: Module
title: ignore
description: "One explicit form for \"this `Result` is deliberately not actionable\"."
resource: crates/oxide-app/src/ignore.rs
tags:
  - "lang:rust"
  - "type:Module"
  - "module:crates"
  - "domain:oxide-app"
  - "git:branch:master"
  - "git:repo:oxide-eda"
timestamp: "2026-08-23T06:04:17Z"
concept_id: crates/oxide-app/src/ignore
language: rust
---

# ignore

One explicit form for "this `Result` is deliberately not actionable".

## Docstring

One explicit form for "this `Result` is deliberately not actionable".

The alternative shapes all have a defect:

* `let _ = expr;` — indistinguishable from an oversight, and the thing
`clippy::let_underscore_must_use` exists to flag. Suppressing it
per site costs a five-line `#[expect(..., reason = "...")]` block.
* `drop(expr);` — silences every lint with no reason recorded, and
`drop` has legitimate uses all over the tree (lock guards), so it
cannot be gated. #577 recorded this as the blind spot with no
clippy answer.

[`IgnoreResult::ignore`] is one line, records the reason, and — the
part that matters — **is implemented only for [`Result`]**. An
`iced::Task` is `#[must_use]` but is not a `Result`, so
`self.handle_x().ignore("…")` does not compile. The escape hatch
`drop(task)` opens stays shut here by construction, which is why this
is a trait on `Result` rather than a free `fn ignore<T>(_: T)`.

It is also greppable in a way an attribute is not: `.ignore("` finds
every deliberate discard in the crate, each with its reason attached.

Not for a failure the user should hear about — that is a bug, and
#533 exists because two of them shipped. Use this only where the
failure genuinely has no consequence and no caller can act on it.

## Relationships

| Type | Target |
|------|--------|
| related | [IgnoreResult](/crates/oxide-app/src/ignore/IgnoreResult.md) |
| related | [ignore](/crates/oxide-app/src/ignore/ignore.md) |
| related | [ignore](/crates/oxide-app/src/ignore/ignore.md) |
| related | [accepts_both_variants](/crates/oxide-app/src/ignore/accepts_both_variants.md) |
| related | [consumes_a_non_copy_payload](/crates/oxide-app/src/ignore/consumes_a_non_copy_payload.md) |
