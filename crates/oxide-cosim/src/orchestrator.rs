//! Hybrid Continuous-Discrete Co-Simulation Orchestrator.

use std::collections::HashMap;
use oxide_mcu::pin_bridge::{LogicLevel, PinBridge, PinFunction};
use oxide_mcu::uart::VirtualUart;
use oxide_proto::ethernet::VirtualEthernetBus;
use oxide_proto::mqtt::{EmbeddedMqttBroker, MqttMessage, QosLevel};
use oxide_types::sim::WaveformDataset;

use crate::session::{CoSimStats, CoSimStatus};

/// Master Co-Simulation Orchestrator tying SPICE analog, QEMU MCU, and virtual network protocol engines.
pub struct CoSimOrchestrator {
    pub status: CoSimStatus,
    pub stats: CoSimStats,
    pub step_time_s: f64,
    pub pin_bridge: PinBridge,
    pub virtual_uart: VirtualUart,
    pub mqtt_broker: EmbeddedMqttBroker,
    pub ethernet_bus: VirtualEthernetBus,
    pub analog_dataset: WaveformDataset,
}

impl CoSimOrchestrator {
    pub fn new(step_time_s: f64) -> Self {
        Self {
            status: CoSimStatus::Idle,
            stats: CoSimStats::default(),
            step_time_s,
            pin_bridge: PinBridge::new(3.3),
            virtual_uart: VirtualUart::new(115200),
            mqtt_broker: EmbeddedMqttBroker::new(),
            ethernet_bus: VirtualEthernetBus::new(),
            analog_dataset: WaveformDataset::empty("Co-Simulation Analog Waveforms"),
        }
    }

    /// Advances the co-simulation timeline by one hybrid step $\Delta t$.
    pub fn step(&mut self) {
        self.stats.sim_time_s += self.step_time_s;
        self.stats.total_steps += 1;

        // 1. Advance MCU cycle count (assuming 168 MHz Cortex-M4)
        let mcu_clock_hz = 168_000_000.0;
        let cycles_this_step = (mcu_clock_hz * self.step_time_s).round() as u64;
        self.stats.mcu_cycles += cycles_this_step;

        // 2. Synchronize boundary pins:
        // Update virtual analog voltages and digital GPIO pin levels
        for pin in self.pin_bridge.pins.values_mut() {
            if pin.function == PinFunction::GpioOutput {
                pin.analog_voltage = pin.level.to_voltage(3.3);
            }
        }
    }

    /// Starts or resumes co-simulation.
    pub fn start(&mut self) {
        self.status = CoSimStatus::Running;
    }

    /// Pauses co-simulation.
    pub fn pause(&mut self) {
        self.status = CoSimStatus::Paused;
    }

    /// Resets co-simulation timeline and virtual peripheral states.
    pub fn reset(&mut self) {
        self.status = CoSimStatus::Idle;
        self.stats = CoSimStats::default();
        self.virtual_uart.clear();
        self.mqtt_broker.clear_log();
        self.ethernet_bus.clear();
    }
}
