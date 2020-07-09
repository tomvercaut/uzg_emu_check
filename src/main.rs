use clap::{crate_authors, crate_description, crate_version, App, Arg};
use console::Term;
use emu_check::EmuError;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::Formatter;
use std::fs::{read, File};
use std::io::{BufRead, BufReader};
use std::option::Option::Some;
use std::process::exit;
use std::slice::SliceIndex;
use log::error;

fn main() {
    simple_logger::init().unwrap();
    println!("EMU check");
    println!("---------");
    let opt_dir_default = dirs::data_local_dir();
    if opt_dir_default.is_none() {
        error!("Unable to determine the local data directory for the current user.");
        exit(1);
    }
    let mut pb_dir_default  = opt_dir_default.unwrap();
    pb_dir_default.push("emu_check");
    let opt_str_dir_default = pb_dir_default.to_str();
    let matches = App::new("emu_check")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("dir")
                .help(
                    "Directory containing the outputfactors and \
                field defining apertures per energy. \
                Each applicator has a seperate csv file for the \
                output factors and field defining apertures.",
                )
                .index(1)
                .required(false)
                .default_value(opt_str_dir_default.unwrap()),
        )
        .get_matches();
    let dirname = matches.value_of("dir").unwrap();
    println!("dirname: {}", dirname);

    // let res_table = read_csv_table(filename_csv);
    // if let Err(e) = res_table {
    //     eprintln!("{}", e.to_string());
    //     exit(1);
    // }
    // let table = res_table.unwrap();
    // let res_of_tables = build_of_tables(&table);
    // if let Err(e) = res_of_tables {
    //     eprintln!("{}", e.to_string());
    //     exit(1);
    // }
    // let of_tables = res_of_tables.unwrap();
    // println!("Input data:");
    // let res_beam_state = ask_questions_beam_state(&of_tables);
    // if let Err(e) = res_beam_state {
    //     eprintln!("{}", e.to_string());
    //     exit(1);
    // }
    // let beam_state = res_beam_state.unwrap();
    // println!("\nBeam paramters:");
    // println!("{}\n", beam_state);
    //
    // println!("Selected OF table:");
    // let res_of_table = filter_of_tables(&of_tables, &beam_state);
    // if let Err(e) = res_of_table {
    //     eprintln!("{}", e.to_string());
    //     exit(1);
    // }
    // let mut of_table = res_of_table.unwrap();
    // // sort by SSD
    // of_table.sort();
    // println!("Selected outputfactor table:");
    // println!("{}", of_table);
    // let res_mu = compute_expected_mu(&of_table, &beam_state);
    // if let Err(e) = res_mu {
    //     eprintln!("{}", e.to_string());
    //     exit(1);
    // }
    // let expected_mu = res_mu.unwrap();
    // println!("Expected MUs: {}", expected_mu);
    // println!(
    //     "Difference [%]: {}",
    //     (1.0 - expected_mu / beam_state.planned_mu) * 100.0
    // );
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub(crate) struct OFTable {
//     pub(crate) machine: String,
//     pub(crate) applicator: String,
//     pub(crate) energy: f64,
//     pub(crate) table: Vec<(f64, f64)>,
// }
//
// impl OFTable {
//     pub(crate) fn new() -> Self {
//         Self {
//             machine: "".to_string(),
//             applicator: "".to_string(),
//             energy: 0.0,
//             table: vec![],
//         }
//     }
//
//     pub(crate) fn sort(&mut self) {
//         self.table.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
//     }
// }
//
// impl std::fmt::Display for OFTable {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         let res = serde_json::to_string_pretty(self);
//         match res {
//             Ok(s) => write!(f, "{}", s),
//             Err(e) => write!(f, "{}", e.to_string()),
//         }
//     }
// }
//
// pub(crate) type OFTables = Vec<OFTable>;
//
// pub(crate) fn filter_of_tables(tables: &OFTables, state: &BeamState) -> Result<OFTable, EmuError> {
//     for table in tables {
//         if (&table.machine == &state.machine)
//             && (&table.applicator == &state.applicator)
//             && (table.energy == state.energy)
//         {
//             return Ok(table.clone());
//         }
//     }
//     return Err(EmuError::OFTableNotFound);
// }
//
// pub(crate) fn interpolate_linear(x: f64, x0: f64, x1: f64, y0: f64, y1: f64) -> f64 {
//     // println!("x: {}", x);
//     // println!("x0: {}", x0);
//     // println!("x1: {}", x1);
//     // println!("y0: {}", y0);
//     // println!("y1: {}", y1);
//     let dx = x1 - x0;
//     if dx <= std::f64::EPSILON {
//         return y0;
//     }
//     y0 + (x - x0) * (y1 - y0) / dx
// }
//
// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub(crate) struct BeamState {
//     pub(crate) machine: String,
//     pub(crate) applicator: String,
//     pub(crate) energy: f64,
//     pub(crate) ssd: f64,
//     pub(crate) ssd_ref: f64,
//     pub(crate) prescription_dose: f64,
//     pub(crate) d2: f64,
//     pub(crate) planned_mu: f64,
// }
//
// impl BeamState {
//     pub(crate) fn new() -> Self {
//         BeamState {
//             machine: "".to_owned(),
//             applicator: "".to_owned(),
//             energy: 0.0,
//             ssd: 0.0,
//             ssd_ref: 0.0,
//             prescription_dose: 0.0,
//             d2: 0.0,
//             planned_mu: 0.0,
//         }
//     }
// }
//
// impl std::fmt::Display for BeamState {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         let res = serde_json::to_string_pretty(self);
//         match res {
//             Ok(s) => write!(f, "{}", s),
//             Err(e) => write!(f, "{}", e.to_string()),
//         }
//     }
// }
//
// pub(crate) fn question(term: &Term, msg: &str) -> Result<String, EmuError> {
//     let mut work = true;
//     while work {
//         if let Err(e) = term.write_str(&format!("{}: ", msg)) {
//             return Err(EmuError::Terminal(e.to_string()));
//         }
//         let res_ans = term.read_line();
//         if let Err(e) = res_ans {
//             return Err(EmuError::Terminal(e.to_string()));
//         }
//         let ans_str = res_ans.unwrap();
//         return Ok(ans_str);
//     }
//     return Err(EmuError::Terminal("Unreachable statement".to_owned()));
// }
//
// pub(crate) fn question_parse_res<T>(term: &Term, msg: &str) -> Result<T, EmuError>
// where
//     T: std::str::FromStr + std::fmt::Display,
//     <T as std::str::FromStr>::Err: std::string::ToString + std::fmt::Debug,
// {
//     let res_ans = question(term, msg);
//     if let Err(e) = res_ans {
//         return Err(e);
//     }
//     let ans_str = res_ans.unwrap();
//     let res_f = ans_str.parse::<T>();
//     if let Err(e) = res_f {
//         return Err(EmuError::Terminal(e.to_string()));
//     }
//     Ok(res_f.unwrap())
// }
//
// pub(crate) fn question_with_options<T: std::fmt::Display>(
//     term: &Term,
//     msg: &str,
//     options: &Vec<T>,
// ) -> Result<usize, EmuError> {
//     let mut work = true;
//     while work {
//         let mut i: usize = 0;
//         for opt in options {
//             if let Err(e) = term.write_line(&format!("{}. {}", i + 1, opt)) {
//                 return Err(EmuError::Terminal(e.to_string()));
//             }
//             i = i + 1;
//         }
//         if let Err(e) = term.write_str(&format!("{}: ", msg)) {
//             return Err(EmuError::Terminal(e.to_string()));
//         }
//         let res_ans = term.read_line();
//         if let Err(e) = res_ans {
//             return Err(EmuError::Terminal(e.to_string()));
//         }
//         let ans_str = res_ans.unwrap();
//         let res_ans_int = ans_str.parse::<usize>();
//         if let Err(e) = res_ans_int {
//             return Err(EmuError::Terminal(e.to_string()));
//         }
//         let ans = res_ans_int.unwrap();
//         if ans >= 1 || ans <= options.len() {
//             work = false;
//             return Ok(ans - 1);
//         }
//     }
//     return Err(EmuError::Terminal("Unreachable statement".to_owned()));
// }
//
// pub(crate) fn ask_questions_beam_state(tables: &OFTables) -> Result<BeamState, EmuError> {
//     let term = Term::stdout();
//     let mut state = BeamState::new();
//     let mut filtered = tables.clone();
//     {
//         let mut machines = vec![];
//         for table in &filtered {
//             if !machines.contains(&table.machine) {
//                 machines.push(table.machine.clone())
//             }
//         }
//         let ans = question_with_options(&term, "Machine", &machines)?;
//         state.machine = machines.get(ans).unwrap().clone();
//         filtered.retain(|table| &table.machine == &state.machine);
//     }
//     {
//         let mut energies = vec![];
//         for table in &filtered {
//             if !energies.contains(&table.energy) {
//                 energies.push(table.energy);
//             }
//         }
//         let ans = question_with_options(&term, "Energy [MeV]", &energies)?;
//         state.energy = energies.get(ans).unwrap().clone();
//         filtered.retain(|table| table.energy == state.energy);
//     }
//     {
//         let mut applicators = vec![];
//         for table in &filtered {
//             if !applicators.contains(&table.applicator) {
//                 applicators.push(table.applicator.clone());
//             }
//         }
//         let ans = question_with_options(&term, "Applicator", &applicators)?;
//         state.applicator = applicators.get(ans).unwrap().clone();
//         filtered.retain(|table| table.applicator == state.applicator);
//     }
//     if filtered.len() != 1 {
//         return Err(EmuError::Logic(format!("Expected only one outputfactor table based on the previous questions but instead found {}", filtered.len())));
//     }
//     state.ssd_ref = question_parse_res(&term, "Reference SSD [cm]")?;
//     state.ssd = question_parse_res(&term, "SSD [cm]")?;
//     state.prescription_dose = question_parse_res(&term, "Prescription [cGy]")?;
//     state.d2 = question_parse_res(&term, "D2 [cGy]")?;
//     state.planned_mu = question_parse_res(&term, "MU plan")?;
//     Ok(state)
// }
//
// pub(crate) fn read_csv_table(filename: &str) -> Result<Vec<Vec<String>>, EmuError> {
//     let file = File::open(filename);
//     if let Err(e) = file {
//         return Err(EmuError::Str(e.to_string()));
//     }
//     let reader = BufReader::new(file.unwrap());
//     let mut table = vec![];
//     for (_index, line) in reader.lines().enumerate() {
//         let line = line.unwrap();
//         let vstr: Vec<&str> = line.split(",").collect();
//         let n = vstr.len();
//         if n > 0 {
//             let mut vs = Vec::with_capacity(n);
//             for s in &vstr {
//                 vs.push(s.to_string());
//             }
//             table.push(vs);
//         }
//     }
//     Ok(table)
// }
//
// fn table_at<'a, T>(v: &'a Vec<Vec<T>>, i: usize, j: usize) -> Option<&'a T> {
//     let tv = v.get(i)?;
//     tv.get(j)
// }
//
// pub(crate) fn build_of_tables(v: &Vec<Vec<String>>) -> Result<OFTables, EmuError> {
//     let mut oft = OFTables::new();
//     let nv = v.len();
//     if nv < 3 {
//         return Err(EmuError::Logic(
//             "Input table doesn't have suffient rows for the required header.".to_owned(),
//         ));
//     }
//     let mut nc = std::usize::MAX;
//     for row in v {
//         let tc = row.len();
//         if nc == std::usize::MAX {
//             nc = tc;
//         } else if tc != nc {
//             return Err(EmuError::Format(
//                 "All rows must have the same number of columns.".to_owned(),
//             ));
//         }
//         if tc == 0 {
//             return Err(EmuError::Str("No rows can be empty".to_owned()));
//         }
//     }
//
//     // Check some input
//     let opt_energy_label = table_at(v, 0, 0);
//     match opt_energy_label {
//         None => {
//             return Err(EmuError::Format(
//                 "expected the label Energy at index (0,0)".to_owned(),
//             ));
//         }
//         Some(label) => {
//             if label != "Energy" {
//                 return Err(EmuError::Format(
//                     "expected the label Energy at index (0,0)".to_owned(),
//                 ));
//             }
//         }
//     }
//     let opt_machine_label = table_at(v, 1, 0);
//     match opt_machine_label {
//         None => {
//             return Err(EmuError::Format(
//                 "expected the label Machine at index (1,0)".to_owned(),
//             ));
//         }
//         Some(label) => {
//             if label != "Machine" {
//                 return Err(EmuError::Format(
//                     "expected the label Machine at index (1,0)".to_owned(),
//                 ));
//             }
//         }
//     }
//     let opt_applicator_label = table_at(v, 2, 0);
//     match opt_applicator_label {
//         None => {
//             return Err(EmuError::Format(
//                 "expected the label Applicator - SSD at index (2,0)".to_owned(),
//             ));
//         }
//         Some(label) => {
//             if label != "Applicator - SSD" {
//                 return Err(EmuError::Format(
//                     "expected the label Applicator - SSD at index (2,0)".to_owned(),
//                 ));
//             }
//         }
//     }
//
//     let mut applicator_indices = vec![];
//     let nr = v.len();
//     for r in 3..nr {
//         let val = table_at(v, r, 0).unwrap();
//         if val.contains("x") {
//             applicator_indices.push(r);
//         }
//     }
//     applicator_indices.push(nr);
//     let n_applicators = applicator_indices.len();
//
//     for a in 0..n_applicators - 1 {
//         // println!("a = {}", a);
//         // println!("index[a] = {}", applicator_indices.get(a).unwrap().clone());
//
//         let applicator_name = table_at(v, applicator_indices.get(a).unwrap().clone(), 0).unwrap();
//         let na0 = applicator_indices.get(a).unwrap().clone() + 1;
//         let na1 = applicator_indices.get(a + 1).unwrap().clone();
//         for c in 1..nc {
//             let mut of_table = OFTable::new();
//             let energy: &String = table_at(v, 0, c).unwrap();
//             let machine: &String = v.get(1).unwrap().get(c).unwrap();
//             of_table.energy = energy.parse::<f64>().unwrap();
//             of_table.machine = machine.clone();
//             of_table.applicator = applicator_name.clone();
//             for r in na0..na1 {
//                 let ssd_str = table_at(v, r, 0).unwrap();
//                 let of_str = table_at(v, r, c).unwrap();
//                 if !of_str.is_empty() {
//                     let ssd = ssd_str.parse::<f64>().unwrap();
//                     let outputfactor = of_str.parse::<f64>().unwrap();
//                     of_table.table.push((ssd, outputfactor));
//                 }
//             }
//             oft.push(of_table);
//         }
//     }
//
//     Ok(oft)
// }
//
// /// Interpolate the output factor in function of the SSD
// pub(crate) fn interpolate_output_factor(
//     of_table: &OFTable,
//     state: &BeamState,
// ) -> Result<f64, EmuError> {
//     let mut min_ssd = std::f64::MAX;
//     let mut max_ssd = std::f64::MIN;
//     let mut mi = std::usize::MAX;
//     let mut mj = std::usize::MAX;
//     let mut di = std::f64::MAX;
//     let mut da = std::f64::MAX;
//     for (i, p) in of_table.table.iter().enumerate() {
//         if p.0 < min_ssd {
//             min_ssd = p.0;
//         }
//         if p.0 > max_ssd {
//             max_ssd = p.0;
//         }
//         let delta = (p.0 - state.ssd).abs();
//         if p.0 <= state.ssd && delta < di {
//             mi = i;
//             di = delta;
//         }
//         if p.0 >= state.ssd && delta < da {
//             mj = i;
//             da = delta;
//         }
//     }
//     if state.ssd < min_ssd || state.ssd > max_ssd {
//         return Err(EmuError::Str(format!(
//             "Requested SSD [{}] is outside of the boundaries of the output factor table: [{},{}]",
//             state.ssd, min_ssd, max_ssd
//         )));
//     }
//     if mi == mj {
//         return Ok(of_table.table.get(mi).unwrap().1);
//     }
//     let y = interpolate_linear(
//         state.ssd,
//         of_table.table.get(mi).unwrap().0,
//         of_table.table.get(mj).unwrap().0,
//         of_table.table.get(mi).unwrap().1,
//         of_table.table.get(mj).unwrap().1,
//     );
//     return Ok(y);
// }
//
// pub(crate) fn compute_expected_mu(
//     of_table: &OFTable,
//     beam_state: &BeamState,
// ) -> Result<f64, EmuError> {
//     let output_factor = interpolate_output_factor(of_table, &beam_state)?;
//     let r = beam_state.d2 / beam_state.prescription_dose;
//     let inv_square = beam_state.ssd * beam_state.ssd / (beam_state.ssd_ref * beam_state.ssd_ref);
//     let mu = beam_state.prescription_dose * inv_square * r / output_factor;
//     Ok(mu)
// }
//
// #[cfg(test)]
// mod test {
//     use super::*;
//
//     fn build_one_of_table() -> OFTable {
//         let mut of_table = OFTable::new();
//         of_table.table.push((115.0, 0.442));
//         of_table.table.push((95.5, 0.856));
//         of_table.table.push((100.0, 0.736));
//         of_table.table.push((98.0, 0.792));
//         of_table.table.push((110.0, 0.526));
//         of_table.table.push((105.0, 0.619));
//         of_table.table.push((95.0, 0.865));
//         of_table
//     }
//
//     fn build_8mev_6x6_of_tabel() -> OFTable {
//         let mut of_table = OFTable::new();
//         of_table.table.push((115.0, 0.584));
//         of_table.table.push((95.5, 0.986));
//         of_table.table.push((100.0, 0.865));
//         of_table.table.push((98.0, 0.919));
//         of_table.table.push((110.0, 0.663));
//         of_table.table.push((105.0, 0.753));
//         of_table.table.push((95.0, 0.994));
//         of_table
//     }
//
//     #[test]
//     fn test_interpolate_output_factor() {
//         let of_table = build_one_of_table();
//         let mut bs = BeamState::new();
//         // test exact matches
//         bs.ssd = 98.0;
//         let res_of1 = interpolate_output_factor(&of_table, &bs);
//         assert!(res_of1.is_ok());
//         assert_eq!(res_of1.unwrap(), 0.792);
//         // test min and max boundaries
//         bs.ssd = 94.9;
//         let res_of2 = interpolate_output_factor(&of_table, &bs);
//         assert!(res_of2.is_err());
//         bs.ssd = 115.1;
//         let res_of3 = interpolate_output_factor(&of_table, &bs);
//         assert!(res_of3.is_err());
//
//         // test interpolation
//         bs.ssd = 106.0;
//         let res_of4 = interpolate_output_factor(&of_table, &bs);
//         assert!(res_of4.is_ok());
//         assert!((res_of4.unwrap() - 0.6004).abs() < std::f64::EPSILON);
//
//         bs.ssd = 109.9;
//         let res_of5 = interpolate_output_factor(&of_table, &bs);
//         assert!(res_of5.is_ok());
//         assert!((res_of5.unwrap() - 0.52786) < std::f64::EPSILON);
//     }
//
//     #[test]
//     fn test_compute_mu1() {
//         let of_table = build_8mev_6x6_of_tabel();
//         let mut bs = BeamState::new();
//         bs.prescription_dose = 800.0;
//         bs.ssd_ref = 95.0;
//         bs.ssd = 98.85;
//         bs.planned_mu = 1030.86;
//         bs.energy = 8.0;
//         bs.d2 = 878.666666666667;
//         let res_mu = compute_expected_mu(&of_table, &bs);
//         assert!(res_mu.is_ok());
//         let mu = res_mu.unwrap();
//         println!("mu: {}", mu);
//         assert!((mu - 1065.79432986395).abs() < std::f64::EPSILON);
//     }
// }
