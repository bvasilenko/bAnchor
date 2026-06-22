use crate::{BanchorError, InductArgs, InductionState, corpus_index::TaskClassCorpusIndex};
use bsuite_core::{HostContext, prompt_resolver::DirectiveString};

pub fn run(
    args: &InductArgs,
    corpus: &TaskClassCorpusIndex,
    _host_context: HostContext,
) -> Result<(DirectiveString, InductionState), BanchorError> {
    validate_task_description(&args.task)?;

    let directive = corpus.resolve(args.task_class).clone();

    Ok((directive, InductionState::Unanchored))
}

fn validate_task_description(task: &str) -> Result<(), BanchorError> {
    if task.trim().is_empty() {
        return Err(BanchorError::MalformedTaskDescription(
            "task must not be empty".to_owned(),
        ));
    }
    Ok(())
}
