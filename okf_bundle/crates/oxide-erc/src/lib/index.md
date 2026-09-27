# lib

## Classs

- [RuleKind](RuleKind.md) — What kind of violation this is. Stable identifier that maps to a severity
- [Severity](Severity.md) — Severity level — maps to colours and sort order in the Messages panel.
- [Violation](Violation.md) — A concrete violation emitted by a rule run. `location` is in world-space

## Functions

- [default_severity](default_severity.md) — Default severity. Users can override per-rule in the Preferences
- [default_severity](default_severity_1.md) — Default severity. Users can override per-rule in the Preferences
- [label](label.md) — Human-readable rule name for the Messages panel and preferences UI.
- [label](label_1.md) — Human-readable rule name for the Messages panel and preferences UI.
- [run](run.md) — Run every enabled rule against the snapshot. Returns a flat list of
- [run_with_dsl](run_with_dsl.md) — Run built-in ERC rules plus caller-provided DSL evaluator functions.
- [run_with_project](run_with_project.md) — Run ERC for a schematic in the context of a whole project. Cross-sheet
- [run_with_project_and_dsl](run_with_project_and_dsl.md) — Run project-scoped ERC with built-in rules plus caller-provided DSL rules.
- [sel](sel.md) — Helper for rules to build a [`SelectedItem`] when they only have a uuid.
