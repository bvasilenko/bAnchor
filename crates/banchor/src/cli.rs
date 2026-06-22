use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

use crate::{MissionAnchorRef, TaskClass};

#[derive(Debug, Parser)]
#[command(name = "banchor")]
#[command(
    about = "CLI mission-rail anchor for agentic loops. Reads task class; emits next-step directive."
)]
pub struct BanchorCli {
    #[command(subcommand)]
    pub cmd: Cmd,
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
    /// Free-form description of the task to anchor.
    #[arg(allow_hyphen_values = true)]
    pub task: String,
    /// Task class from the closed taxonomy (run `banchor task-classes` to list).
    #[arg(long = "task-class")]
    pub task_class: TaskClass,
    /// Named alias or filesystem path to the mission document.
    #[arg(long, allow_hyphen_values = true)]
    pub mission: Option<MissionAnchorRef>,
    /// Repeatable key=value evidence pairs.
    #[arg(long = "evidence", value_parser = parse_evidence_pair)]
    pub evidence: Vec<(String, String)>,
    #[arg(long)]
    pub manifest: Option<PathBuf>,
    /// Emit a JSON envelope instead of plain text.
    #[arg(long)]
    pub json: bool,
    #[arg(long)]
    pub quiet: bool,
    #[arg(long)]
    pub reason: Option<String>,
}

pub fn parse_evidence_pair(value: &str) -> Result<(String, String), String> {
    let (key, val) = value
        .split_once('=')
        .ok_or_else(|| "expected evidence in id=value form".to_owned())?;

    if key.trim().is_empty() {
        return Err("evidence id must not be empty".to_owned());
    }

    Ok((key.to_owned(), val.to_owned()))
}
