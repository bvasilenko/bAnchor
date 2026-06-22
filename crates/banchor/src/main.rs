mod invocation;
mod runtime;

use banchor::{BanchorCli, BanchorError, Cmd};
use bsuite_core::{
    BsuiteCoreError, EmitFormat, ExitCode, ProcessExitEmitter, prompt_resolver::DirectiveString,
};
use clap::Parser;
use invocation::InvocationTranscript;
use runtime::BinaryRuntime;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let cli = BanchorCli::parse();
    let format = emit_format_for(&cli.cmd);
    let mut emitter = ProcessExitEmitter::new(format);

    let exit_code = match init_and_run(cli) {
        Ok(CommandOutcome::Directive {
            directive,
            exit_code,
        }) => emitter.emit_directive(Ok((directive, exit_code))),
        Ok(CommandOutcome::Silent(exit_code)) => exit_code,
        Err(RunError::Malformed(e)) => {
            let _ = writeln!(std::io::stderr(), "{e}");
            ExitCode::Usage
        }
        Err(RunError::Internal(e)) => emitter.emit_directive(Err(e)),
    };

    std::process::exit(exit_code.as_i32());
}

fn init_and_run(cli: BanchorCli) -> Result<CommandOutcome, RunError> {
    let runtime = BinaryRuntime::init(install_dir()).map_err(RunError::Internal)?;
    let invocation = InvocationTranscript::start(
        runtime.host_context,
        runtime.invocation_context.clone(),
        runtime.corpus_version,
    );
    run(cli, runtime, invocation)
}

fn run(
    cli: BanchorCli,
    runtime: BinaryRuntime,
    invocation: InvocationTranscript,
) -> Result<CommandOutcome, RunError> {
    match cli.cmd {
        Cmd::Induct(args) => {
            let result = banchor::induct::run(&args, &runtime.corpus, runtime.host_context)
                .map_err(classify_banchor_error);
            let exit_code = result
                .as_ref()
                .map_or_else(|e| e.exit_code(), |(_, state)| ExitCode::from(*state));
            invocation.flush(&runtime.appender, exit_code, result.is_ok());
            result.map(|(directive, state)| CommandOutcome::Directive {
                directive,
                exit_code: ExitCode::from(state),
            })
        }

        Cmd::TaskClasses => {
            let listing = banchor::TaskClass::ALL
                .iter()
                .map(|tc| tc.to_string())
                .collect::<Vec<_>>()
                .join("\n");
            invocation.flush(&runtime.appender, ExitCode::Success, false);
            Ok(CommandOutcome::Directive {
                directive: DirectiveString::new(listing),
                exit_code: ExitCode::Success,
            })
        }

        Cmd::Update => {
            let mut stderr = std::io::stderr();
            let result = banchor::update::run(&runtime.install_dir, &mut stderr)
                .map_err(|e| RunError::Internal(e.into_core()));
            let exit_code = result
                .as_ref()
                .map_or_else(|e| e.exit_code(), |()| ExitCode::Success);
            invocation.flush(&runtime.appender, exit_code, false);
            result.map(|()| CommandOutcome::Silent(ExitCode::Success))
        }

        Cmd::Init | Cmd::Tail | Cmd::Explain => {
            invocation.flush(&runtime.appender, ExitCode::Success, false);
            Ok(CommandOutcome::Silent(ExitCode::Success))
        }
    }
}

fn emit_format_for(cmd: &Cmd) -> EmitFormat {
    match cmd {
        Cmd::Induct(args) if args.json => EmitFormat::Json,
        _ => EmitFormat::Plain,
    }
}

fn install_dir() -> PathBuf {
    std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(PathBuf::from))
        .unwrap_or_else(|| PathBuf::from("."))
}

fn classify_banchor_error(e: BanchorError) -> RunError {
    if e.is_malformed_input() {
        RunError::Malformed(e)
    } else {
        RunError::Internal(e.into_core())
    }
}

#[derive(Debug)]
enum CommandOutcome {
    Directive {
        directive: DirectiveString,
        exit_code: ExitCode,
    },
    Silent(ExitCode),
}

#[derive(Debug)]
enum RunError {
    Malformed(BanchorError),
    Internal(BsuiteCoreError),
}

impl RunError {
    fn exit_code(&self) -> ExitCode {
        match self {
            Self::Malformed(_) => ExitCode::Usage,
            Self::Internal(_) => ExitCode::InternalError,
        }
    }
}
