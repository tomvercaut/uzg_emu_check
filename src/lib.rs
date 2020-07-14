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

pub fn get_calc_param_input_interactive(
    vcd: &Vec<CorrectionData>,
) -> Result<(CalcParam, CorrectionData), EmuError> {
    let mut calc_param = CalcParam::new();
    // Check if multiple machines are present
    let term = Term::stdout();
    let mut machines = vec![];
    let mut vfcd = vcd.clone();
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
    }
    // else if nmachines == 1 {
    //     machine = machines.get(0).unwrap().clone();
    // }
    else {
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

    // Filter correction data by machine
    // vfcd.retain(|cd| cd.machine == calc_param.machine && cd.applicator == calc_param.applicator);

    // Now only one should remain in the vector
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
    for energy in &cd.output_factors.energies {
            if !venergy.contains(energy) {
                venergy.push(*energy);
            }
        }
    let nve = venergy.len();
    let mut energy = std::f64::MIN;
    if nve == 0 {
        return Err(EmuError::Str(
            "No energy found in the filtered correction data".to_owned(),
        ));
    } else {
        let idx = question_with_options(&term, "Select energy: ", &venergy)?;
        energy = venergy.get(idx).unwrap().clone();
    }
    if !venergy.contains(&energy) {
        return Err(EmuError::Str("No valid energy was selected".to_owned()));
    }

    // Get fda_id
    let mut vfda = vec![];
    let nfda = cd.fda.ids.len();
    if cd.fda.names.len() != nfda {

    }

    // Get source to skin distance
    let ssd: f64 = question_parse_res(&term, "SSD")?;

    unimplemented!()
}
