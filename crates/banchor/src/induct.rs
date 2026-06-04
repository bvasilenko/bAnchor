use std::process::ExitCode;

use bsuite_core::RoutingKey;

use crate::{
    BanchorError, InductArgs, induction_state::InductionState, output, prompt_text,
    taxonomy::TaskClass,
};

const BINARY_NAME: &str = "banchor";
const PLACEHOLDER_TASK_CLASS: TaskClass = TaskClass::Draft;
const PLACEHOLDER_INDUCTION_STATE: InductionState = InductionState::Unanchored;
const MISSION_REF_NONE: &str = "none";

pub fn run(args: InductArgs) -> Result<ExitCode, BanchorError> {
    validate_task_description(&args.task)?;

    let directive = PlaceholderDirective::from_args(&args);
    output::write_stdout_text(&directive)?;

    Ok(process_exit_code(directive.induction_state))
}

struct PlaceholderDirective<'a> {
    task: &'a str,
    task_class: TaskClass,
    mission_ref: Option<&'a crate::MissionAnchorRef>,
    induction_state: InductionState,
    routing_key: RoutingKey,
}

impl<'a> PlaceholderDirective<'a> {
    fn from_args(args: &'a InductArgs) -> Self {
        Self {
            task: &args.task,
            task_class: PLACEHOLDER_TASK_CLASS,
            mission_ref: args.mission.as_ref(),
            induction_state: PLACEHOLDER_INDUCTION_STATE,
            routing_key: RoutingKey::banchor(),
        }
    }

    fn mission_ref_text(&self) -> String {
        self.mission_ref
            .map(ToString::to_string)
            .unwrap_or_else(|| MISSION_REF_NONE.to_owned())
    }

    fn safe_task_text(&self) -> String {
        prompt_text::lossless_field_value(self.task)
    }

    fn safe_mission_ref_text(&self) -> String {
        prompt_text::lossless_field_value(&self.mission_ref_text())
    }
}

impl std::fmt::Display for PlaceholderDirective<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            formatter,
            "[{BINARY_NAME} placeholder directive - pre-corpus output]"
        )?;
        writeln!(
            formatter,
            "Parsed task: {}. Routing key: TaskClass::{:?} placeholder route. Mission ref: {}. Induction-state: {:?}.",
            self.safe_task_text(),
            self.task_class,
            self.safe_mission_ref_text(),
            self.induction_state
        )?;
        writeln!(
            formatter,
            "ACTION: This invocation reached {} at the pre-corpus phase through {}. A real evolved directive would walk the calling LLM through the Mission -> Goals -> Anchors -> Values -> Task -> Expected-result -> Action-items induction ladder for this specific TaskClass.",
            BINARY_NAME,
            self.routing_key.stable_name()
        )?;
        write!(
            formatter,
            "Re-invoke after the corpus-backed release lands. Do not treat this placeholder as ground truth. Exit code carries the verdict-class signal."
        )
    }
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

fn process_exit_code(state: InductionState) -> ExitCode {
    let code = bsuite_core::ExitCode::from(state);
    ExitCode::from(code.as_i32() as u8)
}
