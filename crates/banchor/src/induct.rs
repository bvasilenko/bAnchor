use std::process::ExitCode;

use bsuite_core::RoutingKey;

use crate::{BanchorError, InductArgs, induction_state::InductionState};

pub fn run(args: InductArgs) -> Result<ExitCode, BanchorError> {
    validate_task_description(&args.task)?;
    let _routing_key = RoutingKey::banchor();
    let _default_state = InductionState::Anchored;
    // Deferred until the bsuite-core package exposes verdict wiring.
    println!("not yet implemented");
    Ok(ExitCode::SUCCESS)
}

fn validate_task_description(task: &str) -> Result<(), BanchorError> {
    if task.trim().is_empty() {
        Err(BanchorError::MalformedTaskDescription(
            "task must not be empty".to_owned(),
        ))
    } else {
        Ok(())
    }
}
