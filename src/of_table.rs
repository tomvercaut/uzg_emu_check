use crate::errors::EmuError;
use crate::ipol::interpolate_linear;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OFTable {
    pub(crate) energies: Vec<f64>,
    pub(crate) zrefs: Vec<f64>,
    pub(crate) ssds: Vec<f64>,
    pub(crate) table: Vec<Vec<f64>>, // table.get(i) gets the i th column
                                     // in the table [each column contains the output factors for one energy].
}

impl OFTable {
    pub fn new() -> Self {
        OFTable {
            energies: vec![],
            zrefs: vec![],
            ssds: vec![],
            table: vec![],
        }
    }

    pub fn set_energies(&mut self, values: Vec<f64>) {
        self.energies = values;
    }

    pub fn set_zrefs(&mut self, values: Vec<f64>) {
        self.zrefs = values;
    }

    // Add a list of output factors (one per energy) for a given SSD.
    pub fn add_output_factor_per_ssd(&mut self, ssd: f64, ofs: Vec<f64>) -> Result<(), EmuError> {
        if ofs.len() != self.energies.len() {
            return Err(EmuError::Str(format!(
                "Mismatch between the number energies [{}] and the number of outputfactors [{}]",
                self.energies.len(),
                ofs.len()
            )));
        }
        if ofs.len() != self.zrefs.len() {
            return Err(EmuError::Str(format!(
                "Mismatch between the number zrefs [{}] and the number of outputfactors [{}]",
                self.zrefs.len(),
                ofs.len()
            )));
        }
        self.ssds.push(ssd);
        if self.table.is_empty() {
            for _ in 0..self.energies.len() {
                self.table.push(vec![]);
            }
        }
        let n = ofs.len();
        for ic in 0..n {
            let opt_col = self.table.get_mut(ic);
            if opt_col.is_none() {
                return Err(EmuError::Logic(format!(
                    "Expected table to have a column available at index [{}]",
                    ic
                )));
            }
            let col = opt_col.unwrap();
            // add output factor for the corresponding SSD.
            col.push(*ofs.get(ic).unwrap());
        }
        Ok(())
    }

    // Get the output factor correction based on the energy and the source to skin distance [SSD].
    pub fn get_cf(&self, energy: f64, ssd: f64) -> Result<f64, EmuError> {
        // find matching energy
        let mut energy_idx = self.energies.len();
        for idx in 0..self.energies.len() {
            if energy == *self.energies.get(idx).unwrap() {
                energy_idx = idx;
                break;
            }
        }
        if self.energies.len() == energy_idx {
            return Err(EmuError::EnergyNotFound(energy));
        }

        // Found a matching energy, interpolate output factor by SSD
        let opt_ofs = self.table.get(energy_idx);
        if opt_ofs.is_none() {
            return Err(EmuError::Logic(
                "Energy matching outputfactor column in table was not found.".to_string(),
            ));
        }
        let ofs = opt_ofs.unwrap();
        let n = ofs.len();
        if n != self.ssds.len() {
            return Err(EmuError::Logic(
                "Number of SSDs differs from the number of outputfactors.".to_string(),
            ));
        }

        // look for the closest SSD match
        let mut x0 = std::f64::MIN;
        let mut x1 = std::f64::MAX;
        let mut y0 = std::f64::MAX;
        let mut y1 = std::f64::MAX;
        let mut dx0 = std::f64::MAX;
        let mut dx1 = std::f64::MAX;
        for i in 0..n {
            let issd = self.ssds.get(i).unwrap();
            let dx = (*issd - ssd).abs();
            if dx <= dx0 && *issd <= ssd {
                x0 = *issd;
                y0 = *ofs.get(i).unwrap();
                dx0 = dx;
            }
            if dx <= dx1 && *issd >= ssd {
                x1 = *issd;
                y1 = *ofs.get(i).unwrap();
                dx1 = dx;
            }
        }
        if x0 == std::f64::MIN {
            return Err(EmuError::SSDNotFound(ssd));
        }
        if x1 == std::f64::MAX {
            return Err(EmuError::SSDNotFound(ssd));
        }
        Ok(interpolate_linear(ssd, x0, x1, y0, y1))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn build_of_table() -> OFTable {
        let mut table = OFTable::new();
        table.set_energies(vec![4.0, 6.0, 8.0, 10.0, 12.0]);
        table.set_zrefs(vec![0.89, 1.36, 1.81, 2.31, 2.78]);
        assert!(table
            .add_output_factor_per_ssd(95.0, vec![0.865, 0.953, 0.994, 1.006, 1.037])
            .is_ok());
        assert!(table
            .add_output_factor_per_ssd(95.5, vec![0.856, 0.945, 0.986, 0.995, 1.026])
            .is_ok());
        assert!(table
            .add_output_factor_per_ssd(96.0, vec![0.843, 0.931, 0.973, 0.982, 1.011])
            .is_ok());
        assert!(table
            .add_output_factor_per_ssd(97.0, vec![0.818, 0.902, 0.946, 0.957, 0.982])
            .is_ok());
        assert!(table
            .add_output_factor_per_ssd(98.0, vec![0.792, 0.874, 0.919, 0.932, 0.953])
            .is_ok());
        assert!(table
            .add_output_factor_per_ssd(99.0, vec![0.764, 0.846, 0.892, 0.906, 0.926])
            .is_ok());
        assert!(table
            .add_output_factor_per_ssd(100.0, vec![0.736, 0.818, 0.865, 0.88, 0.899])
            .is_ok());
        assert!(table
            .add_output_factor_per_ssd(105.0, vec![0.619, 0.704, 0.753, 0.775, 0.791])
            .is_ok());
        assert!(table
            .add_output_factor_per_ssd(110.0, vec![0.526, 0.613, 0.663, 0.688, 0.706])
            .is_ok());
        assert!(table
            .add_output_factor_per_ssd(115.0, vec![0.442, 0.533, 0.584, 0.614, 0.63])
            .is_ok());
        table
    }

    #[test]
    fn test_build() {
        let of_table = build_of_table();
        assert_eq!(*of_table.energies.get(0).unwrap(), 4.0);
        assert_eq!(*of_table.energies.get(1).unwrap(), 6.0);
        assert_eq!(*of_table.zrefs.get(0).unwrap(), 0.89);
        assert_eq!(*of_table.zrefs.get(1).unwrap(), 1.36);
    }

    #[test]
    fn test_get_cf() {
        let of_table = build_of_table();
        assert_eq!(of_table.get_cf(4.0, 97.0).unwrap(), 0.818);
        assert_eq!(of_table.get_cf(4.0, 97.3).unwrap(), 0.8102);
        assert_eq!(of_table.get_cf(6.0, 97.0).unwrap(), 0.902);
        assert!((of_table.get_cf(6.0, 97.3).unwrap() - 0.8936) < std::f64::EPSILON);
        assert_eq!(of_table.get_cf(12.0, 97.0).unwrap(), 0.982);
        assert!((of_table.get_cf(12.0, 97.3).unwrap() - 0.9733) < std::f64::EPSILON);

        // fail on purpose
        assert!(of_table.get_cf(3.0, 97.0).is_err());
        assert!(of_table.get_cf(7.0, 97.0).is_err());
        assert!(of_table.get_cf(13.0, 97.0).is_err());
        assert!(of_table.get_cf(8.0, 94.9).is_err());
        assert!(of_table.get_cf(8.0, 115.1).is_err());
    }
}
