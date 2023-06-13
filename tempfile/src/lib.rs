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
    async fn probe(&self, _runtime: Runtime) -> Result<Probe, ActionError> {
        Ok(Probe {
            needs_run: true,
            can_rollback: false
        })
    }

    async fn run(&self, runtime: Runtime, _op: Operation) -> Result<Option<ActionOutput>, ActionError> {
        let contents = match &self.contents {
            ActionInput::Static(contents) => contents.clone(),
            ActionInput::Dynamic(contents) => runtime.get_output(contents.clone())
                .await
                .ok_or(ActionError::NoActionReturn)?
                .try_into()?
        };

        let temp_dir = std::env::temp_dir();
        let name = format!("barley-{}", uuid::Uuid::new_v4());
        let temp_file = temp_dir.join(name);

        std::fs::write(&temp_file, contents)
            .map_err(|e| ActionError::ActionFailed(
                format!("Failed to write to file: {}", e),
                format!("Failed to write to file: {}", e)
            ))?;
        
        Ok(Some(ActionOutput::String(temp_file.to_string_lossy().to_string())))
    }

    fn display_name(&self) -> String {
        "TempFile".to_owned()
    }
}