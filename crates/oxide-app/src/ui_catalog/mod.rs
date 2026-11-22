//! The Altium UI command catalog and registry.
//!
//! This module turns the decompiled Altium `.rcs` resource files into Rust
//! data. It is the Phase-0 foundation for 1:1 UI parity: every command, caption,
//! icon, and shortcut in `altium_ui_catalog.json` is loadable and dispatchable
//! by its Altium name.
//!
//! ## Layout
//! - [`catalog`] — the `UiCommand` / `UiCatalog` data types loaded from the
//!   embedded JSON (3004 commands across 39 `.rcs` files).
//! - [`registry`] — the `CommandRegistry` that dispatches any command by
//!   name to a registered handler.

pub mod catalog;
pub mod registry;

pub use catalog::{UiCatalog, UiCommand};
pub use registry::{CommandRegistry, CommandHandler, DispatchResult, NotImplementedHandler};

#[cfg(test)]
mod tests {
    use super::*;

    /// Phase-0 exit criterion: the command registry can dispatch any of the
    /// 3004 commands by name (never panicking).
    #[test]
    fn phase0_exit_criterion_registry_dispatches_all_commands_by_name() {
        let reg = CommandRegistry::new();
        assert_eq!(reg.catalog_count(), 3004);
        // Every distinct command name must dispatch without error.
        let (ok, not_impl, err) = reg.dispatch_all();
        assert_eq!(err, 0, "every command must be dispatchable without error");
        assert_eq!(ok + not_impl, reg.unique_command_count());
    }

    /// A few spot-checks that well-known Altium commands are present and
    /// dispatchable by their exact names.
    #[test]
    fn well_known_commands_dispatch_by_name() {
        let reg = CommandRegistry::new();
        for name in [
            "PCB:Undo",
            "PCB:Redo",
            "PCB:Zoom",
            "PCB:Select",
            "@PLSCH:InteractiveZoomIn",
        ] {
            assert!(reg.dispatch(name, "").is_not_implemented() || reg.dispatch(name, "").is_ok(),
                "{name} should be dispatchable");
        }
    }
}
