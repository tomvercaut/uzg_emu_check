use crate::errors::EmuError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FdaTable {
    pub(crate) names: Vec<String>, // size of the fitment
    pub(crate) ids: Vec<usize>,
    pub(crate) energies: Vec<f64>,
    pub(crate) table: Vec<Vec<f64>>, // table.get(i) gets the i th column
                                     // in the table [each column contains the output factors for one energy].
}

impl FdaTable {
    pub fn new() -> Self {
        Self {
            names: vec![],
            ids: vec![],
            energies: vec![],
            table: vec![],
        }
    }

    pub fn set_energies(&mut self, values: Vec<f64>) {
        self.energies = values;
    }

    pub fn add(&mut self, name: &str, id: usize, corrections: Vec<f64>) -> Result<(), EmuError> {
        let n = self.energies.len();
        if n != corrections.len() {
            return Err(EmuError::Str(format!(
                "Mismatch between the number energies [{}] and the number of correction factors [{}]",
                self.energies.len(),
                corrections.len()
            )));
        }
        if self.table.is_empty() {
            for _ in 0..n {
                self.table.push(vec![]);
            }
        }
        self.names.push(name.to_string());
        self.ids.push(id);
        for ic in 0..n {
            let opt_col = self.table.get_mut(ic);
            if opt_col.is_none() {
                return Err(EmuError::Logic(format!(
                    "Expected table to have a column availble at index [{}]",
                    ic
                )));
            }
            let col = opt_col.unwrap();
            col.push(*corrections.get(ic).unwrap());
        }
        Ok(())
    }

    // Get the correction factor based on the field defining aperture.
    pub fn get_cf(&self, energy: f64, fda_id: usize) -> Result<f64, EmuError> {
        let nenergies = self.energies.len();
        let mut energy_idx = nenergies;
        for idx in 0..nenergies {
            if energy == *self.energies.get(idx).unwrap() {
                energy_idx = idx;
                break;
            }
        }
        if nenergies == energy_idx {
            return Err(EmuError::EnergyNotFound(energy));
        }

        // Found a matching energy, get the correction factor by fda ID
        let nids = self.ids.len();
        let mut fda_idx = nids;
        for idx in 0..nids {
            if fda_id == *self.ids.get(idx).unwrap() {
                fda_idx = idx;
                break;
            }
        }
        if nids == fda_idx {
            return Err(EmuError::FdaIDNotFound(fda_id));
        }

        let opt_col = self.table.get(energy_idx);
        if opt_col.is_none() {
            return Err(EmuError::Logic(
                "Energy matching correction factor column in table was not found.".to_string(),
            ));
        }
        let col = opt_col.unwrap();
        let opt_cf = col.get(fda_idx);
        if opt_cf.is_none() {
            return Err(EmuError::Logic(
                "FDA id matching correction factor column in table was not found".to_string(),
            ));
        }
        let cf = opt_cf.unwrap();
        Ok(*cf)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn build_fda_table() -> FdaTable {
        let mut fda_table = FdaTable::new();
        fda_table.set_energies(vec![4.0, 6.0, 8.0, 10.0, 12.0]);
        assert!(fda_table
            .add("6x6", 1, vec![0.9, 0.8, 0.7, 0.6, 0.5])
            .is_ok());
        assert!(fda_table
            .add("4x6", 3, vec![1.9, 1.8, 1.7, 1.6, 1.5])
            .is_ok());
        assert!(fda_table
            .add("4x4", 10, vec![2.9, 2.8, 2.7, 2.6, 2.5])
            .is_ok());
        fda_table
    }

    #[test]
    fn fda_table_get_cf() {
        let fda_table = build_fda_table();
        assert_eq!(fda_table.get_cf(6.0, 1).unwrap(), 0.8);
        assert_eq!(fda_table.get_cf(6.0, 3).unwrap(), 1.8);
        assert_eq!(fda_table.get_cf(6.0, 10).unwrap(), 2.8);

        assert_eq!(fda_table.get_cf(8.0, 1).unwrap(), 0.7);
        assert_eq!(fda_table.get_cf(8.0, 3).unwrap(), 1.7);
        assert_eq!(fda_table.get_cf(8.0, 10).unwrap(), 2.7);
    }
}
