use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalcParam {
    pub machine: String,
    pub applicator: String,
    pub energy: f64,
    pub ssd: f64,
    pub depth_zref: f64,
    pub dose_zref: f64,
    pub planned_beam_mu: f64,
    pub fda_id: usize,
}

impl CalcParam {
    pub fn new() -> Self {
        Self {
            machine: "".to_string(),
            applicator: "".to_string(),
            energy: 0.0,
            ssd: 0.0,
            depth_zref: 0.0,
            dose_zref: 0.0,
            fda_id: std::usize::MAX,
        }
    }
}
