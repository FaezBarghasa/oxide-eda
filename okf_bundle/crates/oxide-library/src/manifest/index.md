# manifest

## Classs

- [LibraryMeta](LibraryMeta.md) — [derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
- [LibraryMode](LibraryMode.md) — [derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
- [Manifest](Manifest.md) — [derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
- [TableConfig](TableConfig.md) — Optional `[[tables]]` override block — Altium DBLib parity.
- [UserEntry](UserEntry.md) — [derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
- [UsersConfig](UsersConfig.md) — [derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
- [WorkflowConfig](WorkflowConfig.md) — [derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
- [WorkflowMode](WorkflowMode.md) — Per-library workflow mode — controls which versioning + cascade

## Functions

- [default](default.md)
- [default](default_1.md)
- [default_auto_promote](default_auto_promote.md)
- [default_reviewers_required](default_reviewers_required.md)
- [default_role](default_role.md)
- [parse](parse.md)
- [parse](parse_1.md)
- [parses_database_manifest](parses_database_manifest.md) — [test]
- [parses_local_git_manifest](parses_local_git_manifest.md) — [test]
- [round_trip_preserves_workflow_defaults](round_trip_preserves_workflow_defaults.md) — [test]
- [table_for_class](table_for_class.md) — Resolve a class name to its table filename stem.
- [table_for_class](table_for_class_1.md) — Resolve a class name to its table filename stem.
- [table_for_class_defaults_are_mechanical_plural](table_for_class_defaults_are_mechanical_plural.md) — `table_for_class` defaults are pure suffix pluralisation — irregulars
- [tables](tables.md) — Configured `[[tables]]` overrides, in declaration order.
- [tables](tables_1.md) — Configured `[[tables]]` overrides, in declaration order.
- [tables_overrides_round_trip_and_resolve](tables_overrides_round_trip_and_resolve.md) — Step 1.5 from the plan: tables overrides round-trip through TOML and
- [workflow_mode_defaults_to_personal_when_absent](workflow_mode_defaults_to_personal_when_absent.md) — Missing `[workflow]` block — `mode` defaults to `Personal`
- [workflow_mode_round_trips](workflow_mode_round_trips.md) — Workflow mode round-trips through TOML — Stage 13 of
- [write](write.md)
- [write](write_1.md)
