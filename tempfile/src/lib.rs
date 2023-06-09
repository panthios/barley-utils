use barley_runtime::prelude::*;


pub struct TempFile {
    contents: ActionInput<String>
}

impl TempFile {
    pub fn new<C>(contents: C) -> Self
    where
        C: Into<ActionInput<String>>,
    {
        Self { contents: contents.into() }
    }
}

#[async_trait]
impl Action for TempFile {
    async fn check(&self, _ctx: Runtime) -> Result<bool, ActionError> {
        Ok(false)
    }

    async fn perform(&self, ctx: Runtime) -> Result<Option<ActionOutput>, ActionError> {
        let contents = match self.contents {
            ActionInput::Static(ref contents) => contents.clone(),
            ActionInput::Dynamic(ref action) => {
                ctx.get_output(action.clone()).await
                    .ok_or(ActionError::NoActionReturn)?
                    .try_into()?
            }
        };

        let temp_file = std::env::temp_dir().join(rand::random::<u64>().to_string());
        std::fs::write(temp_file.clone(), contents).map_err(|e| {
            ActionError::ActionFailed(
                format!("Failed to write temp file: {}", temp_file.to_str().unwrap()),
                e.to_string()
            )
        })?;

        Ok(Some(ActionOutput::String(temp_file.to_str().unwrap().to_owned())))
    }

    async fn rollback(&self, _ctx: Runtime) -> Result<(), ActionError> {
        Ok(())
    }

    fn display_name(&self) -> String {
        "TempFile".to_owned()
    }
}