use crate::errors::EmuError;
use crate::fda_table::FdaTable;
use crate::of_table::OFTable;
use crate::{read_fda_table, read_of_table};
use serde::{Deserialize, Serialize};

use async_std::task;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorrectionData {
    pub machine: String,
    pub applicator: String,
    pub output_factors: OFTable,
    pub fda: FdaTable,
}

impl CorrectionData {
    pub fn new() -> Self {
        Self {
            machine: "".to_string(),
            applicator: "".to_string(),
            output_factors: OFTable::new(),
            fda: FdaTable::new(),
        }
    }

    pub fn set_energies(&mut self, values: Vec<f64>) {
        self.output_factors.set_energies(values.clone());
        self.fda.set_energies(values);
    }

    pub fn set_zrefs(&mut self, values: Vec<f64>) {
        self.output_factors.set_zrefs(values);
    }

    pub fn validate(&self) -> bool {
        self.output_factors.get_energies() == self.fda.get_energies()
    }

    pub fn add_output_factor_per_ssd(&mut self, ssd: f64, ofs: Vec<f64>) -> Result<(), EmuError> {
        self.output_factors.add_output_factor_per_ssd(ssd, ofs)
    }

    pub fn add_field_defining_aperture(
        &mut self,
        name: &str,
        id: usize,
        corrections: Vec<f64>,
    ) -> Result<(), EmuError> {
        self.fda.add(name, id, corrections)
    }

    /// Compute the total correction factor: CF_OF * CF_fda
    pub fn get_correction_factor(
        &self,
        energy: f64,
        ssd: f64,
        fda_id: usize,
    ) -> Result<f64, EmuError> {
        let cf_of = self.output_factors.get_cf(energy, ssd)?;
        let cf_fda = self.fda.get_cf(energy, fda_id)?;
        let cf = cf_of * cf_fda;
        Ok(cf)
    }

    pub fn get_energies(&self) -> Vec<f64> {
        self.output_factors.energies.clone()
    }

    pub fn get_energies_as_ref(&self) -> &Vec<f64> {
        &self.output_factors.energies
    }
}

impl Default for CorrectionData {
    fn default() -> Self {
        CorrectionData::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn build_corr_table() -> CorrectionData {
        let mut table = CorrectionData::new();
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

        assert!(table
            .add_field_defining_aperture("6x6", 1, vec![0.9, 0.8, 0.7, 0.6, 0.5])
            .is_ok());
        assert!(table
            .add_field_defining_aperture("4x6", 3, vec![1.9, 1.8, 1.7, 1.6, 1.5])
            .is_ok());
        assert!(table
            .add_field_defining_aperture("4x4", 10, vec![2.9, 2.8, 2.7, 2.6, 2.5])
            .is_ok());
        table
    }

    #[test]
    fn correction_data_get_cf() {
        let table = build_corr_table();
        assert!(table.get_correction_factor(12.0, 103.0, 3).is_ok());
        assert!(
            (table.get_correction_factor(12.0, 103.0, 3).unwrap() - 1.2513) < std::f64::EPSILON
        );
        assert!((table.get_correction_factor(10.0, 96.7, 3).unwrap() - 1.5432) < std::f64::EPSILON);

        assert!(table.get_correction_factor(11.0, 95.0, 3).is_err());
        assert!(table.get_correction_factor(12.0, 94.9, 3).is_err());
        assert!(table.get_correction_factor(12.0, 115.1, 3).is_err());
        assert!(table.get_correction_factor(12.0, 115.0, 4).is_err());
    }
}

fn get_list_data_files(dirname: &str) -> Result<(Vec<PathBuf>, Vec<PathBuf>), EmuError> {
    let dir = PathBuf::from(dirname);
    if !dir.is_dir() {
        return Err(EmuError::DirNotFound(dir));
    }
    let mut vof = vec![];
    let mut vfda = vec![];
    for entry in std::fs::read_dir(dir)? {
        if let Err(e) = entry {
            return Err(EmuError::IO(e.to_string()));
        }
        let entry = entry?;
        let ep = entry.path();
        if ep.is_dir() {
            continue;
        }
        let file_name = entry.file_name();
        let file_name = file_name.to_str().unwrap_or("");
        if file_name.starts_with("of_") {
            vof.push(ep);
        } else if file_name.starts_with("fda_") {
            vfda.push(ep);
        }
    }
    Ok((vof, vfda))
}

/// Load the configuration data (outputfactors and field defining apertures)
/// and process the data into a vector of CorrectionData.
pub async fn load_data(dirname: &str) -> Result<Vec<CorrectionData>, EmuError> {
    let (vof, vfda) = get_list_data_files(dirname)?;
    let nvof = vof.len();
    let nvfda = vfda.len();

    if nvof != nvfda {
        return Err(EmuError::Logic("Number of files with output factors must be identical to the number of files with field defining apertures.".to_owned()
        ));
    }

    // Collect the result on the receiver end
    let mut vof_tables = Vec::with_capacity(nvof);
    let mut vfda_tables = Vec::with_capacity(nvfda);

    let mut thandles_of = vec![];
    let mut thandles_fda = vec![];

    // Spawn a bunch of tasks to read the outputfactor files one by one.
    // Each task returns a handle to a future result containing the data.
    // This allows the result and or it's errors to be passed so it can be
    // proccessed accordingly.
    for pb in vof {
        let tpb = pb.clone();
        thandles_of.push(task::spawn(async move { read_of_table(tpb) }));
    }

    for pb in vfda {
        let tpb = pb.clone();
        thandles_fda.push(task::spawn(async move { read_fda_table(tpb) }));
    }

    // The for loop takes ownership and waits for the result
    // before pushing it in the vector.
    for handle in thandles_of {
        vof_tables.push(handle.await?);
    }
    for handle in thandles_fda {
        vfda_tables.push(handle.await?);
    }

    let mut vcd = vec![];
    for i in 0..nvof {
        let mut cd = CorrectionData::new();
        {
            let (machine, applicator, of_table) = vof_tables.get(i).unwrap();
            cd.machine = machine.clone();
            cd.applicator = applicator.clone();
            cd.output_factors = of_table.clone();
        }
        for j in 0..nvfda {
            let (machine, applicator, fda_table) = vfda_tables.get(j).unwrap();
            if *machine == cd.machine
                && *applicator == cd.applicator
                && fda_table.get_energies() == cd.output_factors.get_energies()
            {
                cd.fda = fda_table.clone();
            }
        }
        if !cd.validate() {
            return Err(EmuError::Logic(
                "Mismatch between the energies in the output factor \
                            table and the field defining aperture table."
                    .to_owned(),
            ));
        }
        vcd.push(cd);
    }

    if vcd.is_empty() {
        return Err(EmuError::IO("No configuration data was loaded.".to_owned()));
    }

    Ok(vcd)
}
