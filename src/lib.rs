// #![allow(dead_code)]
mod calc_param;
pub use calc_param::*;
mod correction_data;
pub use correction_data::*;
mod errors;
pub use errors::*;
mod fda_table;
pub use fda_table::*;
mod correction_data_set;
pub use correction_data_set::*;
mod ipol;
mod of_table;
pub use of_table::*;

use console::Term;

fn question(term: &Term, msg: &str) -> Result<String, EmuError> {
    if let Err(e) = term.write_str(&format!("{}: ", msg)) {
        return Err(EmuError::Terminal(e.to_string()));
    }
    let res_ans = term.read_line();
    if let Err(e) = res_ans {
        return Err(EmuError::Terminal(e.to_string()));
    }
    let ans_str = res_ans.unwrap();
    Ok(ans_str)
}

fn question_parse_res<T>(term: &Term, msg: &str) -> Result<T, EmuError>
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

fn question_with_options<T: std::fmt::Display>(
    term: &Term,
    msg: &str,
    options: &[T],
) -> Result<usize, EmuError> {
    loop {
        if let Err(e) = term.write_line(&format!("{}: ", msg)) {
            return Err(EmuError::Terminal(e.to_string()));
        }
        for (i, opt) in options.iter().enumerate() {
            if let Err(e) = term.write_line(&format!("{}. {}", i + 1, opt)) {
                return Err(EmuError::Terminal(e.to_string()));
            }
        }
        if let Err(e) = term.write_str("Select: ") {
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

/// Obtain the calculation parameters by interactively asking the user for input.
/// The commandline questions are only asked if the corresponding input parameter doesn't contain
/// the data.
/// Return these parameters and the selected correction data based on those parameters.
pub fn get_calc_param_input_cli<'a, 'b>(
    vcd: &'a [CorrectionData],
    opt_input_params: Option<&'b CalcParam>,
) -> Result<(CalcParam, &'a CorrectionData), EmuError> {
    let mut calc_param = CalcParam::new();
    let has_opt_input_param = opt_input_params.is_some();
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
    } else if has_opt_input_param && opt_input_params.unwrap().has_machine() {
        calc_param.machine = opt_input_params.unwrap().machine.clone();
    } else {
        let idx = question_with_options(&term, "Machine", &machines)?;
        calc_param.machine = machines.get(idx).unwrap().clone();
        if let Err(e) = term.write_line("") {
            return Err(EmuError::Terminal(e.to_string()));
        }
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
    } else if has_opt_input_param && opt_input_params.unwrap().has_applicator() {
        calc_param.applicator = opt_input_params.unwrap().applicator.clone();
    } else {
        let idx = question_with_options(&term, "Applicator[cm x cm]", &vapp)?;
        calc_param.applicator = vapp.get(idx).unwrap().clone();
        if let Err(e) = term.write_line("") {
            return Err(EmuError::Terminal(e.to_string()));
        }
    }

    // After filtering by machine and applicator only one match should be found in the vector.
    let nvcd = vcd.len();
    let mut idx = nvcd;
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
    } else if has_opt_input_param
        && opt_input_params.unwrap().has_energy()
        && opt_input_params.unwrap().has_depth_zref()
    {
        let tinput_param = opt_input_params.unwrap();
        calc_param.energy = tinput_param.energy;
        calc_param.depth_zref = tinput_param.depth_zref;
    } else {
        let idx = question_with_options(&term, "Energy[MeV]", &venergy)?;
        calc_param.energy = *venergy.get(idx).unwrap();
        calc_param.depth_zref = *vzref.get(idx).unwrap();
        if let Err(e) = term.write_line("") {
            return Err(EmuError::Terminal(e.to_string()));
        }
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
    } else if opt_input_params.is_some() && opt_input_params.unwrap().has_fda_id() {
        calc_param.fda_id = opt_input_params.unwrap().fda_id;
    } else {
        let idx = question_with_options(&term, "FDA", &vfda)?;
        calc_param.fda_id = *cd.fda.ids.get(idx).unwrap();
        if let Err(e) = term.write_line("") {
            return Err(EmuError::Terminal(e.to_string()));
        }
    }

    if has_opt_input_param && opt_input_params.unwrap().has_ssd() {
        calc_param.ssd = opt_input_params.unwrap().ssd;
    } else {
        // Get source to skin distance
        calc_param.ssd = question_parse_res(&term, "SSD[cm]")?;
        if let Err(e) = term.write_line("") {
            return Err(EmuError::Terminal(e.to_string()));
        }
    }

    if has_opt_input_param && opt_input_params.unwrap().has_dose_zref() {
        calc_param.dose_zref = opt_input_params.unwrap().dose_zref;
    } else {
        // Get the dose at depth zref
        calc_param.dose_zref = question_parse_res(
            &term,
            &*format!("Dose[cGy] (zref: {} cm)", calc_param.depth_zref),
        )?;
        if let Err(e) = term.write_line("") {
            return Err(EmuError::Terminal(e.to_string()));
        }
    }

    if has_opt_input_param && opt_input_params.unwrap().has_planned_beam_mu() {
        calc_param.planned_beam_mu = opt_input_params.unwrap().planned_beam_mu;
    } else {
        // Get the planned MUs in the plan for the beam that's being verified.
        calc_param.planned_beam_mu = question_parse_res(&term, "Planned beam MUs")?;
        if let Err(e) = term.write_line("") {
            return Err(EmuError::Terminal(e.to_string()));
        }
    }

    Ok((calc_param, cd))
}

pub fn calculate_mu(calc_param: &CalcParam, cd: &CorrectionData) -> Result<f64, EmuError> {
    let f = cd.get_correction_factor(calc_param.energy, calc_param.ssd, calc_param.fda_id)?;
    Ok(calc_param.dose_zref / f)
}


pub async fn load_data_calc_mu(
    dirname: &str,
    opt_input_params: Option<&CalcParam>,
) -> Result<(f64, CalcParam), EmuError> {
    let vcd = load_data(dirname).await?;
    let (calc_param, correction_data) = get_calc_param_input_cli(&vcd, opt_input_params)?;
    let mu = calculate_mu(&calc_param, &correction_data)?;
    Ok((mu, calc_param))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calc() {
        let mut vcp = vec![];

        //
        // Applicator 6x6
        //
        vcp.push(CalcParam {
            machine: "Synergy2".to_string(),
            applicator: "6x6".to_string(),
            energy: 12.0,
            ssd: 99.2,
            depth_zref: 2.78,
            dose_zref: 100.0,
            planned_beam_mu: 110.841642761819,
            fda_id: 10,
        });
        vcp.push(CalcParam {
            machine: "Synergy2".to_string(),
            applicator: "6x6".to_string(),
            energy: 12.0,
            ssd: 95.0,
            depth_zref: 2.78,
            dose_zref: 100.0,
            planned_beam_mu: 98.400015744002500,
            fda_id: 10,
        });
        vcp.push(CalcParam {
            machine: "Synergy2".to_string(),
            applicator: "6x6".to_string(),
            energy: 12.0,
            ssd: 95.5,
            depth_zref: 2.78,
            dose_zref: 100.0,
            planned_beam_mu: 99.454986673031800,
            fda_id: 10,
        });
        vcp.push(CalcParam {
            machine: "Synergy2".to_string(),
            applicator: "6x6".to_string(),
            energy: 12.0,
            ssd: 115.0,
            depth_zref: 2.78,
            dose_zref: 100.0,
            planned_beam_mu: 161.969549724652000,
            fda_id: 10,
        });
        vcp.push(CalcParam {
            machine: "Synergy2".to_string(),
            applicator: "6x6".to_string(),
            energy: 12.0,
            ssd: 114.0,
            depth_zref: 2.78,
            dose_zref: 100.0,
            planned_beam_mu: 158.153776079558000,
            fda_id: 10,
        });

        vcp.push(CalcParam {
            machine: "Synergy2".to_string(),
            applicator: "6x6".to_string(),
            energy: 6.0,
            ssd: 99.2,
            depth_zref: 1.36,
            dose_zref: 100.0,
            planned_beam_mu: 118.990956687292000,
            fda_id: 10,
        });
        vcp.push(CalcParam {
            machine: "Synergy2".to_string(),
            applicator: "6x6".to_string(),
            energy: 6.0,
            ssd: 95.0,
            depth_zref: 1.36,
            dose_zref: 100.0,
            planned_beam_mu: 104.931794333683000,
            fda_id: 10,
        });
        vcp.push(CalcParam {
            machine: "Synergy2".to_string(),
            applicator: "6x6".to_string(),
            energy: 6.0,
            ssd: 95.5,
            depth_zref: 1.36,
            dose_zref: 100.0,
            planned_beam_mu: 105.820105820106000,
            fda_id: 10,
        });
        vcp.push(CalcParam {
            machine: "Synergy2".to_string(),
            applicator: "6x6".to_string(),
            energy: 6.0,
            ssd: 115.0,
            depth_zref: 1.36,
            dose_zref: 100.0,
            planned_beam_mu: 187.617260787992000,
            fda_id: 10,
        });
        vcp.push(CalcParam {
            machine: "Synergy2".to_string(),
            applicator: "6x6".to_string(),
            energy: 6.0,
            ssd: 114.0,
            depth_zref: 1.36,
            dose_zref: 100.0,
            planned_beam_mu: 182.149362477231000,
            fda_id: 10,
        });

        //
        // Applicator 10x10
        //

        vcp.push(CalcParam {
            machine: "Synergy2".to_string(),
            applicator: "10x10".to_string(),
            energy: 12.0,
            ssd: 99.2,
            depth_zref: 2.78,
            dose_zref: 100.0,
            planned_beam_mu: 118.046388925549000,
            fda_id: 9,
        });
        vcp.push(CalcParam {
            machine: "Synergy2".to_string(),
            applicator: "10x10".to_string(),
            energy: 12.0,
            ssd: 95.0,
            depth_zref: 2.78,
            dose_zref: 100.0,
            planned_beam_mu: 109.051254089422000,
            fda_id: 9,
        });
        vcp.push(CalcParam {
            machine: "Synergy2".to_string(),
            applicator: "10x10".to_string(),
            energy: 12.0,
            ssd: 95.5,
            depth_zref: 2.78,
            dose_zref: 100.0,
            planned_beam_mu: 109.819994047756000,
            fda_id: 9,
        });
        vcp.push(CalcParam {
            machine: "Synergy2".to_string(),
            applicator: "10x10".to_string(),
            energy: 12.0,
            ssd: 115.0,
            depth_zref: 2.78,
            dose_zref: 100.0,
            planned_beam_mu: 164.729991071634000,
            fda_id: 9,
        });
        vcp.push(CalcParam {
            machine: "Synergy2".to_string(),
            applicator: "10x10".to_string(),
            energy: 12.0,
            ssd: 114.0,
            depth_zref: 2.78,
            dose_zref: 100.0,
            planned_beam_mu: 161.366164678044000,
            fda_id: 9,
        });

        vcp.push(CalcParam {
            machine: "Synergy2".to_string(),
            applicator: "10x10".to_string(),
            energy: 6.0,
            ssd: 99.2,
            depth_zref: 1.36,
            dose_zref: 100.0,
            planned_beam_mu: 110.577390904346000,
            fda_id: 9,
        });
        vcp.push(CalcParam {
            machine: "Synergy2".to_string(),
            applicator: "10x10".to_string(),
            energy: 6.0,
            ssd: 95.0,
            depth_zref: 1.36,
            dose_zref: 100.0,
            planned_beam_mu: 102.040816326531000,
            fda_id: 9,
        });
        vcp.push(CalcParam {
            machine: "Synergy2".to_string(),
            applicator: "10x10".to_string(),
            energy: 6.0,
            ssd: 95.5,
            depth_zref: 1.36,
            dose_zref: 100.0,
            planned_beam_mu: 103.071531642960000,
            fda_id: 9,
        });
        vcp.push(CalcParam {
            machine: "Synergy2".to_string(),
            applicator: "10x10".to_string(),
            energy: 6.0,
            ssd: 115.0,
            depth_zref: 1.36,
            dose_zref: 100.0,
            planned_beam_mu: 157.713781030186000,
            fda_id: 9,
        });
        vcp.push(CalcParam {
            machine: "Synergy2".to_string(),
            applicator: "10x10".to_string(),
            energy: 6.0,
            ssd: 114.0,
            depth_zref: 1.36,
            dose_zref: 100.0,
            planned_beam_mu: 154.186788042506000,
            fda_id: 9,
        });

        //
        // Applicator 14x14
        //

        vcp.push(CalcParam {
            machine: "Synergy2".to_string(),
            applicator: "14x14".to_string(),
            energy: 12.0,
            ssd: 99.2,
            depth_zref: 2.78,
            dose_zref: 100.0,
            planned_beam_mu: 110.815602836879000,
            fda_id: 5,
        });
        vcp.push(CalcParam {
            machine: "Synergy2".to_string(),
            applicator: "14x14".to_string(),
            energy: 12.0,
            ssd: 95.0,
            depth_zref: 2.78,
            dose_zref: 100.0,
            planned_beam_mu: 102.774922918808000,
            fda_id: 5,
        });
        vcp.push(CalcParam {
            machine: "Synergy2".to_string(),
            applicator: "14x14".to_string(),
            energy: 12.0,
            ssd: 95.5,
            depth_zref: 2.78,
            dose_zref: 100.0,
            planned_beam_mu: 104.384133611691000,
            fda_id: 5,
        });
        vcp.push(CalcParam {
            machine: "Synergy2".to_string(),
            applicator: "14x14".to_string(),
            energy: 12.0,
            ssd: 115.0,
            depth_zref: 2.78,
            dose_zref: 100.0,
            planned_beam_mu: 151.515151515152000,
            fda_id: 5,
        });
        vcp.push(CalcParam {
            machine: "Synergy2".to_string(),
            applicator: "14x14".to_string(),
            energy: 12.0,
            ssd: 114.0,
            depth_zref: 2.78,
            dose_zref: 100.0,
            planned_beam_mu: 148.544266191325000,
            fda_id: 5,
        });

        vcp.push(CalcParam {
            machine: "Synergy2".to_string(),
            applicator: "14x14".to_string(),
            energy: 6.0,
            ssd: 99.2,
            depth_zref: 1.36,
            dose_zref: 100.0,
            planned_beam_mu: 109.601052170101000,
            fda_id: 5,
        });
        vcp.push(CalcParam {
            machine: "Synergy2".to_string(),
            applicator: "14x14".to_string(),
            energy: 6.0,
            ssd: 95.0,
            depth_zref: 1.36,
            dose_zref: 100.0,
            planned_beam_mu: 100.806451612903000,
            fda_id: 5,
        });
        vcp.push(CalcParam {
            machine: "Synergy2".to_string(),
            applicator: "14x14".to_string(),
            energy: 6.0,
            ssd: 95.5,
            depth_zref: 1.36,
            dose_zref: 100.0,
            planned_beam_mu: 101.832993890020000,
            fda_id: 5,
        });
        vcp.push(CalcParam {
            machine: "Synergy2".to_string(),
            applicator: "14x14".to_string(),
            energy: 6.0,
            ssd: 115.0,
            depth_zref: 1.36,
            dose_zref: 100.0,
            planned_beam_mu: 152.207001522070000,
            fda_id: 5,
        });
        vcp.push(CalcParam {
            machine: "Synergy2".to_string(),
            applicator: "14x14".to_string(),
            energy: 6.0,
            ssd: 114.0,
            depth_zref: 1.36,
            dose_zref: 100.0,
            planned_beam_mu: 148.986889153754000,
            fda_id: 5,
        });

        //
        // Applicator 20x20
        //

        vcp.push(CalcParam {
            machine: "Synergy2".to_string(),
            applicator: "20x20".to_string(),
            energy: 12.0,
            ssd: 99.2,
            depth_zref: 2.78,
            dose_zref: 100.0,
            planned_beam_mu: 111.656989727557000,
            fda_id: 5,
        });
        vcp.push(CalcParam {
            machine: "Synergy2".to_string(),
            applicator: "20x20".to_string(),
            energy: 12.0,
            ssd: 95.0,
            depth_zref: 2.78,
            dose_zref: 100.0,
            planned_beam_mu: 102.880658436214000,
            fda_id: 5,
        });
        vcp.push(CalcParam {
            machine: "Synergy2".to_string(),
            applicator: "20x20".to_string(),
            energy: 12.0,
            ssd: 95.5,
            depth_zref: 2.78,
            dose_zref: 100.0,
            planned_beam_mu: 103.950103950104000,
            fda_id: 5,
        });
        vcp.push(CalcParam {
            machine: "Synergy2".to_string(),
            applicator: "20x20".to_string(),
            energy: 12.0,
            ssd: 115.0,
            depth_zref: 2.78,
            dose_zref: 100.0,
            planned_beam_mu: 150.602409638554000,
            fda_id: 5,
        });
        vcp.push(CalcParam {
            machine: "Synergy2".to_string(),
            applicator: "20x20".to_string(),
            energy: 12.0,
            ssd: 114.0,
            depth_zref: 2.78,
            dose_zref: 100.0,
            planned_beam_mu: 147.666863555818000,
            fda_id: 5,
        });

        vcp.push(CalcParam {
            machine: "Synergy2".to_string(),
            applicator: "20x20".to_string(),
            energy: 6.0,
            ssd: 99.2,
            depth_zref: 1.36,
            dose_zref: 100.0,
            planned_beam_mu: 108.530497069677000,
            fda_id: 5,
        });
        vcp.push(CalcParam {
            machine: "Synergy2".to_string(),
            applicator: "20x20".to_string(),
            energy: 6.0,
            ssd: 95.0,
            depth_zref: 1.36,
            dose_zref: 100.0,
            planned_beam_mu: 99.502487562189100,
            fda_id: 5,
        });
        vcp.push(CalcParam {
            machine: "Synergy2".to_string(),
            applicator: "20x20".to_string(),
            energy: 6.0,
            ssd: 95.5,
            depth_zref: 1.36,
            dose_zref: 100.0,
            planned_beam_mu: 100.704934541793000,
            fda_id: 5,
        });
        vcp.push(CalcParam {
            machine: "Synergy2".to_string(),
            applicator: "20x20".to_string(),
            energy: 6.0,
            ssd: 115.0,
            depth_zref: 1.36,
            dose_zref: 100.0,
            planned_beam_mu: 148.148148148148000,
            fda_id: 5,
        });
        vcp.push(CalcParam {
            machine: "Synergy2".to_string(),
            applicator: "20x20".to_string(),
            energy: 6.0,
            ssd: 114.0,
            depth_zref: 1.36,
            dose_zref: 100.0,
            planned_beam_mu: 145.137880986938000,
            fda_id: 5,
        });

        for cp in &vcp {
            let res = async_std::task::block_on(load_data_calc_mu("resources", Some(&cp)));
            assert!(res.is_ok());
            let (mu_man, tcp) = res.unwrap();
            assert!(
                (mu_man - cp.planned_beam_mu).abs() < std::f32::EPSILON as f64,
                format!(
                    "CalcParam:{}\nMU[man]={:.15} != MU[plan]={:.15}",
                    tcp, mu_man, cp.planned_beam_mu
                )
            );
        }
    }
}
