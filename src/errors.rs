use serde::{Deserialize, Serialize};
use std::io::Error;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmuError {
    MachineNotFound(String),
    EnergyNotFound(f64),
    SSDNotFound(f64),
    FdaIDNotFound(usize),
    ApplicatorNotFound(String),
    OFTableNotFound,
    Terminal(String),
    Logic(String),
    Str(String),
    Format(String),
    DirNotFound(PathBuf),
    IO(String),
}

impl std::fmt::Display for EmuError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            EmuError::MachineNotFound(machine) => write!(f, "Machine [{}] was not found", &machine),
            EmuError::EnergyNotFound(energy) => write!(f, "Energy [{}] not found", energy),
            EmuError::SSDNotFound(energy) => write!(f, "SSD [{}] is out of range", energy),
            EmuError::FdaIDNotFound(fda_id) => write!(f, "FDA id[{}] not found", fda_id),
            EmuError::ApplicatorNotFound(applicator) => {
                write!(f, "Applicator [{}] not found", &applicator)
            }
            EmuError::OFTableNotFound => write!(f, "OFTable not found"),
            EmuError::Terminal(msg) => write!(f, "Terminal registered an error: {}", msg),
            EmuError::Logic(msg) => write!(f, "{}", msg),
            EmuError::Str(msg) => write!(f, "{}", msg),
            EmuError::Format(msg) => write!(f, "Invalid format: {}", msg),
            EmuError::DirNotFound(path_buf) => {
                write!(f, "Directory not found or does not exist: {:#?}", path_buf)
            }
            EmuError::IO(msg) => write!(f, "Input / output error: {}", msg),
        }
    }
}

impl std::convert::From<std::io::Error> for EmuError {
    fn from(e: Error) -> Self {
        Self::Str(e.to_string())
    }
}
