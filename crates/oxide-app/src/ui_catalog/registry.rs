//! The command registry: loads the full Altium UI catalog and dispatches any
//! of its 3004 commands by name to a registered handler.
//!
//! This is the Phase-0 deliverable that makes 1:1 UI parity tractable. Every
//! command in `altium_ui_catalog.json` is loadable and dispatchable by its
//! Altium name (e.g. `PCB:Undo`). Commands without a live handler dispatch to
//! a default stub that reports `NotImplemented`, so dispatch never panics.

use std::collections::HashMap;

use super::catalog::{UiCatalog, UiCommand};

/// The result of dispatching a command.
#[derive(Debug, Clone)]
pub enum DispatchResult {
    /// The handler executed successfully.
    Ok,
    /// No live handler is registered for this command yet (it exists in the
    /// catalog but is not implemented).
    NotImplemented,
    /// The handler reported an error.
    Err(String),
}

impl DispatchResult {
    pub fn is_ok(&self) -> bool {
        matches!(self, DispatchResult::Ok)
    }

    pub fn is_not_implemented(&self) -> bool {
        matches!(self, DispatchResult::NotImplemented)
    }
}

/// A handler that executes a single Altium command.
///
/// Implementors receive the full [`UiCommand`] metadata (caption, params,
/// description) and the runtime `params` string. They return a
/// [`DispatchResult`] describing the outcome.
pub trait CommandHandler {
    fn execute(&self, command: &UiCommand, params: &str) -> DispatchResult;
}

/// A handler that reports `NotImplemented` for every command. Used as the
/// default so that every catalog command is dispatchable without a live arm.
#[derive(Debug)]
pub struct NotImplementedHandler;

impl CommandHandler for NotImplementedHandler {
    fn execute(&self, _command: &UiCommand, _params: &str) -> DispatchResult {
        DispatchResult::NotImplemented
    }
}

/// The command registry.
///
/// Holds the full Altium UI catalog and a map of Altium command name to handler.
/// Every command in the catalog is dispatchable: commands without an explicit
/// handler fall through to [`NotImplementedHandler`].
pub struct CommandRegistry {
    catalog: UiCatalog,
    handlers: HashMap<String, Box<dyn CommandHandler>>,
}

impl CommandRegistry {
    /// Build a registry from the embedded Altium UI catalog. Every command in
    /// the catalog is pre-registered with a default stub handler so dispatch
    /// never panics.
    pub fn new() -> Self {
        let catalog = UiCatalog::load();
        let mut handlers: HashMap<String, Box<dyn CommandHandler>> = HashMap::new();
        for cmd in catalog.all_commands() {
            handlers.insert(cmd.command.clone(), Box::new(NotImplementedHandler));
        }
        Self { catalog, handlers }
    }

    /// Register (or override) a live handler for a command by its Altium name.
    pub fn register(&mut self, command: &str, handler: impl CommandHandler + 'static) {
        self.handlers.insert(command.to_string(), Box::new(handler));
    }

    /// Look up a command's metadata by its Altium name.
    pub fn get(&self, command: &str) -> Option<&UiCommand> {
        self.catalog.all_commands().find(|c| c.command == command)
    }

    /// Dispatch a command by its Altium name.
    ///
    /// Returns `Err` if the name is not in the catalog at all; otherwise calls
    /// the registered handler (or the default stub) and returns its result.
    pub fn dispatch(&self, command: &str, params: &str) -> DispatchResult {
        let cmd = match self.get(command) {
            Some(cmd) => cmd,
            None => return DispatchResult::Err(format!("unknown command: {command}")),
        };
        match self.handlers.get(command) {
            Some(handler) => handler.execute(cmd, params),
            None => DispatchResult::NotImplemented,
        }
    }

    /// Number of commands in the catalog (3004).
    pub fn catalog_count(&self) -> usize {
        self.catalog.count()
    }

    /// Number of distinct command names (1190).
    pub fn unique_command_count(&self) -> usize {
        self.catalog.unique_command_count()
    }

    /// All distinct command names in the catalog, sorted.
    pub fn all_command_names(&self) -> Vec<String> {
        let names: std::collections::HashSet<String> =
            self.catalog.all_commands().map(|c| c.command.clone()).collect();
        let mut out: Vec<String> = names.into_iter().collect();
        out.sort();
        out
    }

    /// Dispatch every command in the catalog and summarize the results.
    ///
    /// Returns `(ok, not_implemented, err)` counts. Useful as a smoke test
    /// that the whole catalog is dispatchable without panicking.
    pub fn dispatch_all(&self) -> (usize, usize, usize) {
        let mut ok = 0;
        let mut not_impl = 0;
        let mut err = 0;
        for name in self.all_command_names() {
            match self.dispatch(&name, "") {
                DispatchResult::Ok => ok += 1,
                DispatchResult::NotImplemented => not_impl += 1,
                DispatchResult::Err(_) => err += 1,
            }
        }
        (ok, not_impl, err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct EchoHandler;

    impl CommandHandler for EchoHandler {
        fn execute(&self, command: &UiCommand, params: &str) -> DispatchResult {
            assert_eq!(command.command, "PCB:Undo");
            assert_eq!(params, "Action=Toggle|SetFocus=True");
            DispatchResult::Ok
        }
    }

    #[test]
    fn registry_loads_full_catalog() {
        let reg = CommandRegistry::new();
        assert_eq!(reg.catalog_count(), 3004);
        assert_eq!(reg.unique_command_count(), 1190);
    }

    #[test]
    fn dispatch_known_command_returns_not_implemented_by_default() {
        let reg = CommandRegistry::new();
        // PCB:Undo exists in the catalog but has no live handler yet.
        assert!(reg.dispatch("PCB:Undo", "").is_not_implemented());
    }

    #[test]
    fn dispatch_unknown_command_returns_err() {
        let reg = CommandRegistry::new();
        match reg.dispatch("NOPE:NotACommand", "") {
            DispatchResult::Err(_) => {}
            other => panic!("expected Err, got {other:?}"),
        }
    }

    #[test]
    fn register_and_dispatch_live_handler() {
        let mut reg = CommandRegistry::new();
        reg.register("PCB:Undo", EchoHandler);
        assert!(reg.dispatch("PCB:Undo", "Action=Toggle|SetFocus=True").is_ok());
    }

    #[test]
    fn dispatch_all_never_panics_and_covers_every_command() {
        let reg = CommandRegistry::new();
        let (ok, not_impl, err) = reg.dispatch_all();
        assert_eq!(err, 0, "no command should error");
        assert_eq!(ok + not_impl, reg.unique_command_count());
    }

    #[test]
    fn get_returns_metadata() {
        let reg = CommandRegistry::new();
        let cmd = reg.get("PCB:Undo").expect("PCB:Undo in catalog");
        assert_eq!(cmd.namespace(), "PCB");
        assert_eq!(cmd.action(), "Undo");
    }
}
