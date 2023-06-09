use barley_runtime::prelude::*;
use reqwest::get;


pub struct HttpGet {
    url: ActionInput<String>
}

impl HttpGet {
    pub fn new<U>(url: U) -> Self
    where
        U: Into<ActionInput<String>>,
    {
        Self { url: url.into() }
    }
}

#[async_trait]
impl Action for HttpGet {
    async fn check(&self, _ctx: Runtime) -> Result<bool, ActionError> {
        Ok(false)
    }

    async fn perform(&self, ctx: Runtime) -> Result<Option<ActionOutput>, ActionError> {
        let url = match self.url {
            ActionInput::Static(ref url) => url.clone(),
            ActionInput::Dynamic(ref action) => {
                ctx.get_output(action.clone()).await
                    .ok_or(ActionError::NoActionReturn)?
                    .try_into()?
            }
        };

        let res = get(url.clone()).await.map_err(|e| {
            ActionError::ActionFailed(
                format!("Failed to GET URL: {}", url),
                e.to_string()
            )
        })?;

        let body = res.text().await.map_err(|e| {
            ActionError::ActionFailed(
                format!("Failed to GET URL: {}", url),
                e.to_string()
            )
        })?;

        Ok(Some(ActionOutput::String(body)))
    }

    async fn rollback(&self, _ctx: Runtime) -> Result<(), ActionError> {
        Ok(())
    }

    fn display_name(&self) -> String {
        format!("HttpGet {}", match self.url {
            ActionInput::Static(ref url) => url.clone(),
            ActionInput::Dynamic(_) => "<dynamic>".to_owned()
        })
    }
}