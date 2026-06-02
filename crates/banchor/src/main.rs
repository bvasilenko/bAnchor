use std::process::ExitCode;

use banchor::{BanchorCli, dispatch_cli};
use clap::{Parser, error::ErrorKind};

fn main() -> ExitCode {
    match BanchorCli::try_parse() {
        Ok(cli) => match dispatch_cli(cli) {
            Ok(output) => output.exit_code,
            Err(error) => {
                eprintln!("{error}");
                to_process_exit_code(error.exit_code())
            }
        },
        Err(error) => {
            let kind = error.kind();
            let _ = error.print();

            if matches!(kind, ErrorKind::DisplayHelp | ErrorKind::DisplayVersion) {
                ExitCode::SUCCESS
            } else {
                to_process_exit_code(bsuite_core::ExitCode::Usage)
            }
        }
    }
}

fn to_process_exit_code(code: bsuite_core::ExitCode) -> ExitCode {
    ExitCode::from(code.as_i32() as u8)
}
