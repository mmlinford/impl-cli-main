use clap::Parser;

use impl_cli_main::impl_cli_main;

const PASS_INPUT: &str = "pass";
const FAIL_INPUT: &str = "fail";

impl_cli_main!();

fn run(args: &CliArgs) -> Result<(), RunError> {
    match args.input.as_str() {
        PASS_INPUT => Ok(()),
        FAIL_INPUT => Err(RunError::FailInput),
        unrecognized => Err(RunError::UnrecognizedInput(unrecognized.into())),
    }
}

#[derive(Parser)]
struct CliArgs {
    input: String,
}

#[derive(Debug, thiserror::Error)]
enum RunError {
    #[error("program was passed the intentional failure argument {FAIL_INPUT}")]
    FailInput,

    #[error("unrecognized argument: {0}")]
    UnrecognizedInput(String),
}
