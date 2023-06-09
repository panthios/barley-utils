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
    async fn check(&self, _ctx: Runtime) -> Result<bool, ActionError> {
        Ok(false)
    }

    async fn perform(&self, ctx: Runtime) -> Result<Option<ActionOutput>, ActionError> {
        let mut result = String::new();

        for input in &self.0 {
            let value = match input {
                ActionInput::Static(value) => value.clone(),
                ActionInput::Dynamic(action) => ctx.get_output(action.clone()).await
                    .ok_or(ActionError::NoActionReturn)?
                    .try_into()?
            };

            result.push_str(&value);
        }

        Ok(Some(result.into()))
    }

    async fn rollback(&self, _ctx: Runtime) -> Result<(), ActionError> {
        Ok(())
    }

    fn display_name(&self) -> String {
        "".to_string()
    }
}