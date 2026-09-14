//! Universal Synchronous/Asynchronous Receiver Transmitter (USART/UART) Emulation.

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum WordLength {
    #[default]
    Bits8,
    Bits7,
    Bits9,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Parity {
    #[default]
    None,
    Even,
    Odd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum StopBits {
    #[default]
    Bits1,
    Bits0_5,
    Bits1_5,
    Bits2,
}

/// Full UART/USART hardware peripheral model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UartPeripheral {
    pub name: String,
    pub enabled: bool,
    pub tx_enabled: bool,
    pub rx_enabled: bool,
    pub baud_rate: u32,
    pub word_length: WordLength,
    pub parity: Parity,
    pub stop_bits: StopBits,
    pub hw_flow_control: bool,
    pub rs485_driver_enable: bool,
    pub tx_fifo: VecDeque<u8>,
    pub rx_fifo: VecDeque<u8>,
    pub tx_empty: bool,
    pub rx_not_empty: bool,
}

impl UartPeripheral {
    pub fn new(name: impl Into<String>, baud_rate: u32) -> Self {
        Self {
            name: name.into(),
            enabled: true,
            tx_enabled: true,
            rx_enabled: true,
            baud_rate,
            word_length: WordLength::Bits8,
            parity: Parity::None,
            stop_bits: StopBits::Bits1,
            hw_flow_control: false,
            rs485_driver_enable: false,
            tx_fifo: VecDeque::with_capacity(128),
            rx_fifo: VecDeque::with_capacity(128),
            tx_empty: true,
            rx_not_empty: false,
        }
    }

    /// Transmits a byte from MCU firmware.
    pub fn write_byte(&mut self, byte: u8) {
        if self.tx_enabled {
            self.tx_fifo.push_back(byte);
            self.tx_empty = false;
        }
    }

    /// Reads received byte into MCU firmware.
    pub fn read_byte(&mut self) -> Option<u8> {
        let byte = self.rx_fifo.pop_front();
        self.rx_not_empty = !self.rx_fifo.is_empty();
        byte
    }

    /// Injects byte received from physical bus / terminal into RX FIFO.
    pub fn inject_rx_byte(&mut self, byte: u8) {
        if self.rx_enabled {
            self.rx_fifo.push_back(byte);
            self.rx_not_empty = true;
        }
    }

    /// Pops byte from TX FIFO to transmit over physical bus.
    pub fn pop_tx_byte(&mut self) -> Option<u8> {
        let b = self.tx_fifo.pop_front();
        self.tx_empty = self.tx_fifo.is_empty();
        b
    }
}
