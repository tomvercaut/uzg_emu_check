use crate::{load_data, CorrectionData, EmuError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorrectionDataSet {
    data: Vec<CorrectionData>,
}

impl Default for CorrectionDataSet {
    fn default() -> Self {
        CorrectionDataSet::new()
    }
}

impl From<Vec<CorrectionData>> for CorrectionDataSet {
    fn from(v: Vec<CorrectionData>) -> Self {
        Self { data: v }
    }
}

impl From<&Vec<CorrectionData>> for CorrectionDataSet {
    fn from(v: &Vec<CorrectionData>) -> Self {
        Self { data: v.clone() }
    }
}

impl CorrectionDataSet {
    pub fn new() -> Self {
        Self { data: vec![] }
    }

    pub fn get_machines(&self) -> Vec<String> {
        let mut v = vec![];
        for cd in &self.data {
            if !v.contains(&cd.machine) {
                v.push(cd.machine.clone());
            }
        }
        v
    }

    pub fn get_energies(&self, machine: &str) -> Vec<f64> {
        let mut v = vec![];
        for cd in &self.data {
            if cd.machine.as_str() == machine {
                for energy in cd.get_energies_as_ref() {
                    if !v.contains(energy) {
                        v.push(*energy);
                    }
                }
            }
        }
        v
    }

    pub fn get_applicators(&self, machine: &str, energy: f64) -> Vec<String> {
        let mut v = vec![];
        for cd in &self.data {
            if cd.machine.as_str() == machine
                && cd.get_energies_as_ref().contains(&energy)
                && !v.contains(&cd.applicator)
            {
                v.push(cd.applicator.clone());
            }
        }
        v
    }

    pub fn get_applicator_fitments(
        &self,
        machine: &str,
        energy: f64,
        applicator: &str,
    ) -> Vec<String> {
        let mut v = vec![];
        for cd in &self.data {
            if cd.machine.as_str() == machine
                && cd.get_energies_as_ref().contains(&energy)
                && cd.applicator.as_str() == applicator
            {
                for app_fit in &cd.fda.names {
                    if !v.contains(app_fit) {
                        v.push(app_fit.clone());
                    }
                }
            }
        }
        v
    }

    // Get zref in function of the machine, applicator and energy.
    pub fn get_zref<S: AsRef<str> + ?Sized>(
        &self,
        machine: &S,
        applicator: &S,
        energy: &S,
    ) -> Option<f64> {
        if let Ok(energy_) = energy.as_ref().parse::<f64>() {
            for cd in self.data.iter() {
                if cd.machine.as_str() == machine.as_ref()
                    && cd.get_energies().contains(&energy_)
                    && cd.applicator.as_str() == applicator.as_ref()
                {
                    return cd.get_zref(energy_);
                }
            }
        }
        None
    }

    pub fn calc<S: AsRef<str> + ?Sized>(
        machine: &S,
        applicator: &S,
        applicator_fitment: &S,
        energy: &S,
        ssd: &S,
        planned_beam_mu: &S,
        dose_zref: &S
    ) {
        //
    }
}

#[derive(Debug, Clone)]
pub struct Computed {}

/// Load the configuration data (outputfactors and field defining apertures)
/// and process the data into a CorrectionDataSet.
pub async fn correction_data_set_load_data(dirname: &str) -> Result<CorrectionDataSet, EmuError> {
    let res = load_data(dirname).await?;
    Ok(CorrectionDataSet::from(res))
}
