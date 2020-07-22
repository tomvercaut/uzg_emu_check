#![allow(unused_imports)]
use async_std::prelude::*;
use clap::{crate_authors, crate_description, crate_version, App, Arg};
use console::Term;
use emu_check::{calculate_mu, get_calc_param_input_cli, get_list_data_files, question_with_options, read_fda_table, read_of_table, CalcParam, CorrectionData, EmuError, load_data};
use log::error;
use std::process::exit;
use std::sync::mpsc;
use std::thread;

#[async_std::main]
fn main() {
    simple_logger::init().unwrap();
    println!("EMU check");
    println!("---------");
    let opt_dir_default = dirs::data_local_dir();
    if opt_dir_default.is_none() {
        error!("Unable to determine the local data directory for the current user.");
        exit(1);
    }
    let mut pb_dir_default = opt_dir_default.unwrap();
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

    let res_data = load_data(dirname).await;
    if let Err(e) = res_data {
        error!("Error was detected while loading the configuration data: {}", e.to_string());
        exit(1);
    }

    let vcd = res_data.unwrap();

    let res_calc_data = get_calc_param_input_cli(&vcd, &None);
    if let Err(e) = res_calc_data {
        error!("Error while getting input user: {}", e.to_string());
        exit(1);
    }

    let (calc_param, cd) = res_calc_data.unwrap();
    let res_mu = calculate_mu(&calc_param, cd);
    if let Err(e) = res_mu {
        error!("Error while calculating the MU's: {}", e);
        exit(1);
    }
}
