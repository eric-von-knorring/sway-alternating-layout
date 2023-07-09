mod alternating_layout;

extern crate exitcode;

use std::process;
use crate::alternating_layout::start_alternating_layout;

fn main() {
    let mut args = std::env::args().skip(1);

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-p" => {
                let path = args.next()
                    .unwrap_or_else(|| {
                        eprintln!("Error: missing path to pid file");
                        process::exit(exitcode::USAGE);
                    });
                write_pid(path)
            },
            "-h" => print_help_text(),
            _ => unknown_argument(&arg)
        }
    }

    start_alternating_layout();
}

fn print_help_text() {
    println!("Usage: sway-alternating-layout [-p path/to/pid.file]

Options:
    -p path/to/pid.file   Saves the PID for this program in the filename specified")
}

fn unknown_argument(arg: &str) {
    eprintln!("Error: Unknown argument '{}'", arg);
    process::exit(exitcode::USAGE);
}

fn write_pid(path: String) {
    match std::fs::write(path, process::id().to_string()) {
        Ok(_) => {}
        Err(err) => {
            eprintln!("Error: could not write pid. '{}'", err);
            process::exit(exitcode::CANTCREAT);
        }
    }
}