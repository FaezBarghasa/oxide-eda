//! `oxide-cosim` — Multi-Domain Co-Simulation Orchestrator for Oxide EDA.
//!
//! Synchronizes analog SPICE solvers, QEMU virtual microcontroller execution,
//! and discrete IoT/protocol network models onto a unified simulation timeline.

pub mod fpga_bridge;
pub mod orchestrator;
pub mod session;

pub use fpga_bridge::{DigitalLogicState, FpgaBridge, FpgaPin};
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

    #[test]
    fn test_fpga_bridge_cosim() {
        let mut fpga = FpgaBridge::new("ICE40_CORE", "Lattice iCE40-HX8K", 50_000_000);
        fpga.register_pin("CLK", false, Some("NET_CLK".to_string()));
        fpga.register_pin("LED_OUT", true, Some("NET_LED_RGB".to_string()));

        // Step 100 FPGA clock cycles
        fpga.step_cycles(100);
        assert_eq!(fpga.current_cycle, 100);

        // Memory mapped register access
        fpga.write_register(0x00, 0xA5A5_5A5A);
        assert_eq!(fpga.read_register(0x00), 0xA5A5_5A5A);
    }
}

