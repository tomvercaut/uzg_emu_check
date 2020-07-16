#![allow(dead_code)]
mod calc_param;
pub use calc_param::*;
mod correction_data;
pub use correction_data::*;
mod errors;
pub use errors::*;
mod fda_table;
pub use fda_table::*;
mod ipol;
mod of_table;
pub use of_table::*;

use console::Term;
use std::path::PathBuf;

pub fn question(term: &Term, msg: &str) -> Result<String, EmuError> {
    if let Err(e) = term.write_str(&format!("{}: ", msg)) {
        return Err(EmuError::Terminal(e.to_string()));
    }
    let res_ans = term.read_line();
    if let Err(e) = res_ans {
        return Err(EmuError::Terminal(e.to_string()));
    }
    let ans_str = res_ans.unwrap();
    return Ok(ans_str);
}

pub(crate) fn question_parse_res<T>(term: &Term, msg: &str) -> Result<T, EmuError>
where
    T: std::str::FromStr + std::fmt::Display,
    <T as std::str::FromStr>::Err: std::string::ToString + std::fmt::Debug,
{
    let res_ans = question(term, msg);
    if let Err(e) = res_ans {
        return Err(e);
    }
    let ans_str = res_ans.unwrap();
    let res_f = ans_str.parse::<T>();
    if let Err(e) = res_f {
        return Err(EmuError::Terminal(e.to_string()));
    }
    Ok(res_f.unwrap())
}

pub fn question_with_options<T: std::fmt::Display>(
    term: &Term,
    msg: &str,
    options: &Vec<T>,
) -> Result<usize, EmuError> {
    loop {
        let mut i: usize = 0;
        for opt in options {
            if let Err(e) = term.write_line(&format!("{}. {}", i + 1, opt)) {
                return Err(EmuError::Terminal(e.to_string()));
            }
            i = i + 1;
        }
        if let Err(e) = term.write_str(&format!("{}: ", msg)) {
            return Err(EmuError::Terminal(e.to_string()));
        }
        let res_ans = term.read_line();
        if let Err(e) = res_ans {
            return Err(EmuError::Terminal(e.to_string()));
        }
        let ans_str = res_ans.unwrap();
        let res_ans_int = ans_str.parse::<usize>();
        if let Err(e) = res_ans_int {
            return Err(EmuError::Terminal(e.to_string()));
        }
        let ans = res_ans_int.unwrap();
        if ans >= 1 || ans <= options.len() {
            return Ok(ans - 1);
        }
    }
}

pub fn get_list_data_files(dirname: &str) -> Result<(Vec<PathBuf>, Vec<PathBuf>), EmuError> {
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

/// Obtain the calculation parameters by interactively asking the user for input.
/// Return these parameters and the selected correction data based on those parameters.
pub fn get_calc_param_input_interactive(
    vcd: &Vec<CorrectionData>,
) -> Result<(CalcParam, &CorrectionData), EmuError> {
    let mut calc_param = CalcParam::new();
    // Check if multiple machines are present
    let term = Term::stdout();
    let mut machines = vec![];
    for cd in vcd {
        if !machines.contains(&cd.machine) {
            machines.push(cd.machine.clone());
        }
    }
    let nmachines = machines.len();
    if nmachines == 0 {
        return Err(EmuError::Str(
            "No machines found in the correction data.".to_owned(),
        ));
    } else {
        let idx = question_with_options(&term, "Select a machine", &machines)?;
        calc_param.machine = machines.get(idx).unwrap().clone();
    }

    // Get applicator
    let mut vapp = vec![];
    for cd in vcd {
        if !vapp.contains(&cd.applicator) && cd.machine == calc_param.machine {
            vapp.push(cd.applicator.clone());
        }
    }
    let nvapp = vapp.len();
    if nvapp == 0 {
        return Err(EmuError::Str(
            "No applicators found in the correction data.".to_owned(),
        ));
    } else {
        let idx = question_with_options(&term, "Select applicator", &vapp)?;
        calc_param.applicator = vapp.get(idx).unwrap().clone();
    }

    // After filtering by machine and applicator only one match should be found in the vector.
    let nvcd = vcd.len();
    let mut idx = nvcd.clone();
    for i in 0..nvcd {
        let cd = vcd.get(i).unwrap();
        if cd.machine == calc_param.machine && cd.applicator == calc_param.applicator {
            if idx == nvcd {
                idx = i;
            } else {
                return Err(EmuError::Logic(format!(
                    "Multiple correction data matches found for [machine: {}, applicator: {}]",
                    calc_param.machine, calc_param.applicator
                )));
            }
        }
    }

    if idx == nvcd {
        return Err(EmuError::CorrectionDataNotFound(
            calc_param.machine,
            calc_param.applicator,
        ));
    }

    // the selected correction data table
    let cd = vcd.get(idx).unwrap();

    // Get user selected energy
    let mut venergy = vec![];
    let mut vzref = vec![];
    let nenergy = cd.output_factors.energies.len();
    for i in 0..nenergy {
        let energy = cd.output_factors.energies.get(i).unwrap();
        if !venergy.contains(energy) {
            venergy.push(*energy);
            vzref.push(*cd.output_factors.zrefs.get(i).unwrap());
        }
    }
    let nve = venergy.len();
    if nve == 0 {
        return Err(EmuError::Str(
            "No energy found in the filtered correction data".to_owned(),
        ));
    } else {
        let idx = question_with_options(&term, "Select energy", &venergy)?;
        calc_param.energy = venergy.get(idx).unwrap().clone();
        calc_param.depth_zref = vzref.get(idx).unwrap().clone();
    }
    if !venergy.contains(&calc_param.energy) {
        return Err(EmuError::Str("No valid energy was selected".to_owned()));
    }

    // Get fda_id
    let mut vfda = vec![];
    let nfda = cd.fda.ids.len();
    for i in 0..nfda {
        let name = cd.fda.names.get(i).unwrap();
        let id = cd.fda.ids.get(i).unwrap();
        vfda.push(format!("{} [id={}]", name, id));
    }
    if vfda.is_empty() {
        return Err(EmuError::Str(
            "No FDA IDs found in filtered correction data".to_owned(),
        ));
    } else {
        let idx = question_with_options(&term, "Select FDA", &vfda)?;
        calc_param.fda_id = *cd.fda.ids.get(idx).unwrap();
    }

    // Get source to skin distance
    calc_param.ssd = question_parse_res(&term, "SSD")?;

    // Get the dose at depth zref
    calc_param.dose_zref = question_parse_res(
        &term,
        &*format!("Dose (cGy) [zref: {} mm]", calc_param.depth_zref),
    )?;

    // Get the planned MUs in the plan for the beam that's being verified.
    calc_param.plan_mu = question_parse_res(&term, "Planned beam MUs")?;

    Ok((calc_param, cd))
}


pub fn calculate_mu(calc_param: &CalcParam, cd: &CorrectionData) -> Result<f64, EmuError> {
    let f = cd.get_correction_factor(calc_param.energy, calc_param.ssd, calc_param.fda_id)?;
    Ok(calc_param.dose_zref/f)
}
