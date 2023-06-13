use barley_runtime::prelude::*;


pub struct Join(Vec<ActionInput<String>>);

impl Join {
    pub fn new<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = ActionInput<String>>,
    {
        Self(iter.into_iter().collect())
    }
}

#[async_trait]
impl Action for Join {
    async fn probe(&self, _runtime: Runtime) -> Result<Probe, ActionError> {
        Ok(Probe {
            needs_run: true,
            can_rollback: false
        })
    }

    async fn run(&self, runtime: Runtime, op: Operation) -> Result<Option<ActionOutput>, ActionError> {
        if matches!(op, Operation::Rollback) {
            return Err(ActionError::OperationNotSupported);
        }

        let mut output = String::new();

        for input in &self.0 {
            let input = match input {
                ActionInput::Static(input) => input.clone(),
                ActionInput::Dynamic(input) => runtime.get_output(input.clone())
                    .await
                    .ok_or(ActionError::NoActionReturn)?
                    .try_into()?
            };

            output.push_str(&input);
        }

        Ok(Some(ActionOutput::String(output)))
    }

    fn display_name(&self) -> String {
        "".to_string()
    }
}