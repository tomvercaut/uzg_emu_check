#![allow(unused_imports)]
use clap::{crate_authors, crate_description, crate_version, App, Arg};
use console::Term;
use emu_check::{get_list_data_files, read_fda_table, read_of_table, CorrectionData, EmuError, question_with_options};
use log::error;
use std::process::exit;
use std::sync::mpsc;
use std::thread;

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
    let res_list_data_files = get_list_data_files(dirname);
    if let Err(e) = res_list_data_files {
        error!("Unable to get a list of the data files: {}", e.to_string());
        exit(1);
    }
    let (vof, vfda) = res_list_data_files.unwrap();
    let nvof = vof.len();
    let nvfda = vfda.len();

    if nvof != nvfda {
        error!(
            "Number of files with output factors must be identical to the number of files \
        with field defining apertures."
        );
        exit(1);
    }

    // Collect the result on the receiver end
    let mut vof_tables = Vec::with_capacity(nvof);
    let mut vfda_tables = Vec::with_capacity(nvfda);

    // Start reading the outputfactor files one by one.
    // Each file is read in a separate thread.
    // The result is send through a channel to the receiver.
    let (tx_of, rx_of) = mpsc::channel();
    for pb in vof {
        let tpb = pb.clone();
        let ctx_of = tx_of.clone();
        thread::spawn(move || {
            // captures tpb
            let res_of_table = read_of_table(tpb);
            if let Err(e) = res_of_table {
                error!("Unable to read output factor table: {}", e.to_string());
                exit(1);
            }
            let of_table = res_of_table.unwrap();
            if let Err(e) = ctx_of.send(of_table) {
                error!("Channel sender caught an error: {}", e.to_string());
                exit(1);
            }
        });
    }

    // Start reading the field defining aperture files.
    // Each file is read in a seperate thread.
    // The result is send through a channel to the receiver.
    let (tx_fda, rx_fda) = mpsc::channel();
    for pb in vfda {
        let tpb = pb.clone();
        let ctx_fda = tx_fda.clone();
        thread::spawn(move || {
            // capture the transmitter
            let res_fda_table = read_fda_table(tpb);
            if let Err(e) = res_fda_table {
                error!(
                    "Unable to read field defining aperture table: {}",
                    e.to_string()
                );
                exit(1);
            }
            let fda_table = res_fda_table.unwrap();
            if let Err(e) = ctx_fda.send(fda_table) {
                error!("Channel sender caught an error: {}", e.to_string());
                exit(1);
            }
        });
    }

    for _ in 0..nvof {
        let res = rx_of.recv();
        if let Err(e) = res {
            error!("Channel receiver caught an error: {}", e.to_string());
            exit(1);
        }
        vof_tables.push(res.unwrap());
    }

    for _ in 0..nvfda {
        let res = rx_fda.recv();
        if let Err(e) = res {
            error!("Channel receiver caught an error: {}", e.to_string());
            exit(1);
        }
        vfda_tables.push(res.unwrap());
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
            error!(
                "Mismatch between the energies in the output factor \
                table and the field defining aperture table."
            );
            exit(1);
        }
        vcd.push(cd);
    }

    if vcd.is_empty() {
        error!("No configuration data was loaded.");
        exit(1);
    }

    // Check if multiple machines are present
    let term = Term::stdout();
    let mut machine = "".to_string();
    let mut machines = vec![];
    for cd in &vcd {
       if !machines.contains(&cd.machine) { machines.push(cd.machine.clone());}
    }
    let nmachines = machines.len();
    if nmachines == 0 {
        error!("No machines found in the correction data.");
        exit(1);
    } else if nmachines == 1 {
        machine = machines.get(0).unwrap().clone();
    } else if nmachines > 1 {
        let res_idx = question_with_options(&term, "Select a machine", &machines);
        if let Err(e) = res_idx {
            error!("{}", e.to_string());
            exit(1);
        }
        machine = machines.get(res_idx.unwrap()).unwrap().clone();
    }
    for cd in &mut vcd {

    }
}
