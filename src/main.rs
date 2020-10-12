#![allow(unused_imports)]
use async_std::prelude::*;
use async_std::task;
use clap::{crate_authors, crate_description, crate_version, App, Arg};
use console::Term;
use emu_check::{load_data_calc_mu, EmuError};
use log::{error, trace, LevelFilter};
use simple_logger::SimpleLogger;
use std::process::exit;
use std::sync::mpsc;
use std::thread;

#[async_std::main]
async fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();
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
    trace!("dirname: {}", dirname);

    let res = task::block_on(load_data_calc_mu(dirname, None));
    if let Err(e) = res {
        error!("Something went wrong:\n{}", e.to_string());
        exit(1);
    }
    let (mu, calc_param) = res.unwrap();
    let proc_diff = (1.0 - (calc_param.planned_beam_mu / mu)) * 100.0;
    let s = format!(
        "Calculation parameters:\n{}\nMU(check): {:.4}\nDifference[%]: {:.6}",
        calc_param, mu, proc_diff
    );
    println!("{}", s);
}
