//! Virtual UART Serial Terminal FIFO Buffer.

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// Virtual Serial Port FIFO buffer connecting MCU USART/UART to the EDA Console.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VirtualUart {
    pub baud_rate: u32,
    pub tx_buffer: VecDeque<u8>,
    pub rx_buffer: VecDeque<u8>,
    pub history_lines: Vec<String>,
    current_line: String,
}

impl VirtualUart {
    pub fn new(baud_rate: u32) -> Self {
        Self {
            baud_rate,
            tx_buffer: VecDeque::new(),
            rx_buffer: VecDeque::new(),
            history_lines: Vec::new(),
            current_line: String::new(),
        }
    }

    /// Receives a byte emitted by the virtual MCU UART transmitter.
    pub fn push_from_mcu(&mut self, byte: u8) {
        self.tx_buffer.push_back(byte);
        let ch = byte as char;
        if ch == '\n' {
            self.history_lines.push(std::mem::take(&mut self.current_line));
        } else if ch != '\r' {
            self.current_line.push(ch);
        }
    }

    /// Queues user input string to be read by MCU UART receiver.
    pub fn write_from_terminal(&mut self, text: &str) {
        for byte in text.bytes() {
            self.rx_buffer.push_back(byte);
        }
    }

    /// Reads next byte if available for MCU firmware `USART_ReceiveData()`.
    pub fn read_byte_for_mcu(&mut self) -> Option<u8> {
        self.rx_buffer.pop_front()
    }

    /// Clears terminal history.
    pub fn clear(&mut self) {
        self.tx_buffer.clear();
        self.rx_buffer.clear();
        self.history_lines.clear();
        self.current_line.clear();
    }
}
