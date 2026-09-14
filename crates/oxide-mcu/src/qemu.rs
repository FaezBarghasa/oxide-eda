//! QEMU Process Manager & Virtual Machine Supervisor.

use std::path::PathBuf;
use std::process::Stdio;
use tokio::process::{Child, Command};
use thiserror::Error;

use crate::firmware::FirmwareImage;

#[derive(Debug, Error)]
pub enum QemuError {
    #[error("QEMU binary not found on PATH: {0}")]
    BinaryNotFound(String),
    #[error("Failed to spawn QEMU child process: {0}")]
    SpawnFailed(#[from] std::io::Error),
    #[error("QEMU socket connection error: {0}")]
    SocketError(String),
}

/// Configuration for starting virtual MCU target in QEMU.
#[derive(Debug, Clone)]
pub struct QemuConfig {
    pub qemu_bin: String,
    pub firmware: FirmwareImage,
    pub gdb_port: Option<u16>,
    pub serial_socket_path: Option<PathBuf>,
    pub enable_semihosting: bool,
}

impl QemuConfig {
    pub fn new(firmware: FirmwareImage) -> Self {
        let profile = firmware.target.profile();
        Self {
            qemu_bin: profile.qemu_executable.to_string(),
            firmware,
            gdb_port: Some(1234),
            serial_socket_path: None,
            enable_semihosting: true,
        }
    }
}

/// Running QEMU instance supervisor.
pub struct QemuInstance {
    pub config: QemuConfig,
    child: Option<Child>,
}

impl QemuInstance {
    /// Launches the architecture-specific QEMU process in background.
    pub fn spawn(config: QemuConfig) -> Result<Self, QemuError> {
        let profile = config.firmware.target.profile();
        let mut cmd = Command::new(&config.qemu_bin);

        // Machine & CPU args derived from universal CoreProfile
        cmd.arg("-M").arg(profile.qemu_machine);
        cmd.arg("-cpu").arg(profile.qemu_cpu);
        cmd.arg("-kernel").arg(&config.firmware.path);
        cmd.arg("-nographic");

        if config.enable_semihosting {
            cmd.arg("-semihosting");
        }

        if let Some(port) = config.gdb_port {
            cmd.arg("-gdb").arg(format!("tcp::{port}"));
        }

        cmd.stdout(Stdio::piped())
           .stderr(Stdio::piped())
           .stdin(Stdio::piped());

        let child = match cmd.spawn() {
            Ok(c) => c,
            Err(e) => {
                if e.kind() == std::io::ErrorKind::NotFound {
                    return Err(QemuError::BinaryNotFound(config.qemu_bin.clone()));
                }
                return Err(QemuError::SpawnFailed(e));
            }
        };

        Ok(Self {
            config,
            child: Some(child),
        })
    }

    /// Terminates the running QEMU instance.
    pub async fn kill(&mut self) -> Result<(), std::io::Error> {
        if let Some(mut child) = self.child.take() {
            child.kill().await?;
        }
        Ok(())
    }
}

impl Drop for QemuInstance {
    fn drop(&mut self) {
        if let Some(mut child) = self.child.take() {
            let _ = child.start_kill();
        }
    }
}
