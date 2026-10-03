use super::{CommandFlags, CommandGroup, CommandMetadata, DocumentKind, Enablement};

pub(super) const PCB: &[CommandMetadata] = &[
    CommandMetadata {
        id: "run_drc",
        category: "validation",
        label: "Run design rule check",
        menu_label: Some("Design Rule Check"),
        group: CommandGroup::Pcb,
        enable: Enablement::RequiresDocument(DocumentKind::Pcb),
        flags: CommandFlags::NONE,
        ..CommandMetadata::DEFAULT
    },
    CommandMetadata {
        id: "run_simulation",
        category: "simulation",
        label: "Run SPICE simulation",
        menu_label: Some("Run SPICE Simulation"),
        group: CommandGroup::General,
        enable: Enablement::Always,
        flags: CommandFlags::NONE,
        ..CommandMetadata::DEFAULT
    },
];
