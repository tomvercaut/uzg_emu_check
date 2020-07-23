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
            planned_beam_mu: 0.0,
            fda_id: std::usize::MAX,
        }
    }

    pub fn has_machine(&self) -> bool {
        !self.machine.is_empty()
    }

    pub fn has_applicator(&self) -> bool {
        !self.applicator.is_empty()
    }

    pub fn has_energy(&self) -> bool {
        self.energy != 0.0
    }

    pub fn has_ssd(&self) -> bool {
        self.ssd != 0.0
    }

    pub fn has_depth_zref(&self) -> bool {
        self.depth_zref != 0.0
    }

    pub fn has_dose_zref(&self) -> bool {
        self.dose_zref != 0.0
    }

    pub fn has_planned_beam_mu(&self) -> bool {
        self.planned_beam_mu != 0.0
    }

    pub fn has_fda_id(&self) -> bool {
        self.fda_id != std::usize::MAX
    }
}

impl Default for CalcParam {
    fn default() -> Self {
        CalcParam::new()
    }
}

impl std::fmt::Display for CalcParam {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Machine: {}\nApplicator: {}\nEnergy(MeV): {}\nSSD[cm]: {}\nZref(depth[cm]): {}\nZref(dose[cGy]): {}\nFDA ID: {}\nMU(plan): {}\n",
        self.machine, self.applicator, self.energy, self.ssd, self.depth_zref, self.dose_zref, self.fda_id, self.planned_beam_mu
        )
    }
}
