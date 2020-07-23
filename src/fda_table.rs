use crate::errors::EmuError;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

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

    pub fn get_energies(&self) -> &Vec<f64> {
        &self.energies
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
            if (energy - *self.energies.get(idx).unwrap()).abs() < std::f64::EPSILON {
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

impl Default for FdaTable {
    fn default() -> Self {
        FdaTable::new()
    }
}

pub fn read_fda_table(path_buf: PathBuf) -> Result<(String, String, FdaTable), EmuError> {
    let mut fda_table = FdaTable::new();
    let mut machine = "".to_owned();
    let mut applicator = "".to_owned();
    let res_rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(path_buf);
    if let Err(e) = res_rdr {
        return Err(EmuError::IO(e.to_string()));
    }
    let mut rdr = res_rdr.unwrap();
    let mut nc = 0;
    let mut i = 0;
    for record in rdr.records() {
        if let Err(e) = record {
            return Err(EmuError::IO(e.to_string()));
        }
        let record = record.unwrap();
        let nrecord = record.len();
        if nrecord == 0 {
            continue;
        }
        if nc == 0 {
            nc = nrecord;
        }
        if nc != nrecord {
            return Err(EmuError::Format(format!(
                "All rows in the CSV file must have the same number of columns [{} <-> {}]",
                nc, nrecord
            )));
        }
        if i == 0 {
            machine = record[0].to_string();
        } else if i == 1 {
            if &record[0] != "Applicator" {
                return Err(EmuError::Format(
                    "Expected the label \'Applicator\' on row 1, column 0".to_owned(),
                ));
            }
            applicator = record[1].to_string();
        } else if i == 2 {
            if &record[0] != "Dimensions" {
                return Err(EmuError::Format(
                    "Expected the label \'Dimensions\' on row 2, column 0".to_owned(),
                ));
            }
            if &record[1] != "id" {
                return Err(EmuError::Format(
                    "Expected the label \'id\' on row 2, column 1".to_owned(),
                ));
            }
            let mut energies = Vec::with_capacity(nrecord - 1);
            for j in 2..nrecord {
                let s = &record[j];
                let res_f = s.parse::<f64>();
                if let Err(e) = res_f {
                    return Err(EmuError::Format(e.to_string()));
                }
                energies.push(res_f.unwrap());
            }
            fda_table.energies = energies;
        } else {
            let name = &record[0];
            let sid = &record[1];
            let res_id = sid.parse::<usize>();
            if let Err(e) = res_id {
                return Err(EmuError::Format(e.to_string()));
            }
            let mut v = vec![];
            for j in 2..nrecord {
                let s = &record[j];
                let res_f = s.parse::<f64>();
                if let Err(e) = res_f {
                    return Err(EmuError::Format(e.to_string()));
                }
                v.push(res_f.unwrap());
            }
            fda_table.add(name, res_id.unwrap(), v)?;
        }
        i += 1;
    }
    Ok((machine, applicator, fda_table))
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
