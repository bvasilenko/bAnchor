use std::{path::PathBuf, process::ExitCode};

use clap::{Args, Parser, Subcommand};

use crate::{BanchorError, MissionAnchorRef, TaskClass, induct, output};

const PUBLIC_PLACEHOLDER_MESSAGE: &str = "not yet implemented";

#[derive(Debug, Parser)]
#[command(name = "banchor")]
#[command(
    about = "CLI mission-rail anchor for agentic loops. Reads task class; emits next-step directive."
)]
pub struct BanchorCli {
    #[command(subcommand)]
    pub command: Cmd,
}

#[derive(Debug, Subcommand)]
pub enum Cmd {
    Induct(InductArgs),
    TaskClasses,
    Update,
    Init,
    Tail,
    Explain,
}

#[derive(Debug, Clone, Eq, PartialEq, Args)]
pub struct InductArgs {
    pub task: String,
    #[arg(long)]
    pub mission: Option<MissionAnchorRef>,
    #[arg(long = "evidence", value_parser = parse_evidence_pair)]
    pub evidence: Vec<(String, String)>,
    #[arg(long)]
    pub manifest: Option<PathBuf>,
    #[arg(long)]
    pub json: bool,
    #[arg(long)]
    pub quiet: bool,
    #[arg(long)]
    pub reason: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CliOutput {
    pub exit_code: ExitCode,
}

pub fn dispatch_cli(cli: BanchorCli) -> Result<CliOutput, BanchorError> {
    let exit_code = match cli.command {
        Cmd::Induct(args) => induct::run(args)?,
        Cmd::TaskClasses => print_task_classes()?,
        Cmd::Update => placeholder("update")?,
        Cmd::Init => placeholder("init")?,
        Cmd::Tail => placeholder("tail")?,
        Cmd::Explain => placeholder("explain")?,
    };

    Ok(CliOutput { exit_code })
}

pub fn parse_evidence_pair(value: &str) -> Result<(String, String), String> {
    let (key, value) = value
        .split_once('=')
        .ok_or_else(|| "expected evidence in id=value form".to_owned())?;

    if key.trim().is_empty() {
        return Err("evidence id must not be empty".to_owned());
    }

    Ok((key.to_owned(), value.to_owned()))
}

fn print_task_classes() -> Result<ExitCode, BanchorError> {
    output::write_stdout_lines(TaskClass::ALL)?;

    Ok(ExitCode::SUCCESS)
}

fn placeholder(command_name: &'static str) -> Result<ExitCode, BanchorError> {
    output::write_stdout_text(format_args!("{PUBLIC_PLACEHOLDER_MESSAGE}: {command_name}"))?;

    Ok(ExitCode::SUCCESS)
}
