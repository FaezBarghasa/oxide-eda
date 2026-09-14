//! `oxide-cosim` — Multi-Domain Co-Simulation Orchestrator for Oxide EDA.
//!
//! Synchronizes analog SPICE solvers, QEMU virtual microcontroller execution,
//! and discrete IoT/protocol network models onto a unified simulation timeline.

pub mod orchestrator;
pub mod session;

pub use orchestrator::CoSimOrchestrator;
pub use session::{CoSimStats, CoSimStatus};

#[cfg(test)]
mod tests {
    use super::*;
    use oxide_mcu::pin_bridge::{LogicLevel, PinFunction};

    #[test]
    fn test_cosim_step_synchronization() {
        let mut orch = CoSimOrchestrator::new(1e-6); // 1 µs step
        orch.pin_bridge.register_pin("PA5", PinFunction::GpioOutput, Some("NET_LED".to_string()));
        orch.pin_bridge.set_gpio_output("PA5", LogicLevel::High);

        orch.start();
        orch.step();

        assert_eq!(orch.stats.total_steps, 1);
        assert_eq!(orch.stats.sim_time_s, 1e-6);
        assert!(orch.stats.mcu_cycles > 0);

        let pin = orch.pin_bridge.pins.get("PA5").unwrap();
        assert_eq!(pin.analog_voltage, 3.3);
    }
}
