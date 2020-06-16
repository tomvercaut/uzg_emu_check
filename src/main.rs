use console::Term;
use std::error::Error;
use std::fmt::Formatter;
use std::fs::read;
use std::io::Write;
use std::option::Option::Some;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Clone)]
pub(crate) enum EmuError {
    MachineNotFound(String),
    EnergyNotFound(f64),
    ApplicatorNotFound(String),
    OFTableNotFound,
    Terminal(String),
}

impl std::fmt::Display for EmuError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            EmuError::MachineNotFound(machine) => write!(f, "Machine [{}] was not found", &machine),
            EmuError::EnergyNotFound(energy) => write!(f, "Energy [{}] not found", energy),
            EmuError::ApplicatorNotFound(applicator) => {
                write!(f, "Applicator [{}] not found", &applicator)
            }
            EmuError::OFTableNotFound => write!(f, "OFTable not found"),
            EmuError::Terminal(msg) => write!(f, "Terminal registered an error: {}", msg),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct OFTable {
    pub(crate) machine: String,
    pub(crate) applicator: String,
    pub(crate) energy: f64,
    pub(crate) table: Vec<(f64, f64)>,
}

pub(crate) type OFTables = Vec<OFTable>;

pub(crate) fn filter_of_tables(
    tables: &OFTables,
    machine: &str,
    applicator: &str,
    energy: f64,
) -> Result<OFTable, EmuError> {
    for table in tables {
        if (&table.machine == machine)
            && (&table.applicator == applicator)
            && (table.energy == energy)
        {
            return Ok(table.clone());
        }
    }
    return Err(EmuError::OFTableNotFound);
}

pub(crate) fn interpolate_linear(x: f64, x0: f64, x1: f64, y0: f64, y1: f64) -> f64 {
    let dx = x1 - x0;
    if dx == 0.0 {
        return y0;
    }
    y0 + (x - x0) * (y1 - y0) / dx
}

pub(crate) struct BeamState {
    pub(crate) machine: String,
    pub(crate) applicator: String,
    pub(crate) energy: f64,
    pub(crate) ssd: f64,
    pub(crate) ssd_ref: f64,
    pub(crate) prescription_dose: f64,
    pub(crate) d2: f64,
    pub(crate) expected_mu: f64,
}

impl BeamState {
    pub fn new() -> Self {
        BeamState {
            machine: "".to_owned(),
            applicator: "".to_owned(),
            energy: 0.0,
            ssd: 0.0,
            ssd_ref: 0.0,
            prescription_dose: 0.0,
            d2: 0.0,
            expected_mu: 0.0,
        }
    }
}

pub(crate) fn question(term: &Term, msg: &str) -> Result<String, EmuError> {
    let mut work = true;
    while work {
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
    return Err(EmuError::Terminal("Unreachable statement".to_owned()));
}

pub(crate) fn question_parse_res<T>(term: &Term, msg: &str) -> Result<T, EmuError> {
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

pub(crate) fn question_with_options<T: std::fmt::Display>(
    term: &Term,
    msg: &str,
    options: &Vec<T>,
) -> Result<usize, EmuError> {
    let mut work = true;
    while work {
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
            work = false;
            return Ok(ans - 1);
        }
    }
    return Err(EmuError::Terminal("Unreachable statement".to_owned()));
}

pub(crate) fn ask_questions_beam_state(tables: &OFTables) -> Result<BeamState, EmuError> {
    let term = Term::stdout();
    let mut state = BeamState::new();
    let mut filtered = tables.clone();
    {
        let mut machines = vec![];
        for table in &filtered {
            if !machines.contains(&table.machine) {
                machines.push(table.machine.clone())
            }
        }
        let res_ans = question_with_options(&term, "Select machine", &machines);
        if let Err(e) = res_ans {
            return Err(e);
        }
        let ans = question_with_options(&term, "Select machine", &machines)?;
        state.machine = machines.get(ans).unwrap().clone();
        filtered.retain(|table| &table.machine == &state.machine);
    }
    {
        let mut energies = vec![];
        for table in &filtered {
            energies.push(table.energy);
        }
        let ans = question_with_options(&term, "Select energy", &energies)?;
        state.energy = energies.get(ans).unwrap().clone();
        filtered.retain(|table| table.energy == state.energy);
    }
    state.ssd_ref = question_parse_res(&term, "Reference SSD")?;
    state.ssd = question_parse_res(&term, "SSD")?;
    // TODO continue here
    Ok(state)
}
