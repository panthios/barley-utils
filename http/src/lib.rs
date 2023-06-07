use barley_runtime::prelude::*;
use anyhow::anyhow;
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
    async fn check(&self, _ctx: Runtime) -> Result<bool> {
        Ok(false)
    }

    async fn perform(&self, ctx: Runtime) -> Result<Option<ActionOutput>> {
        let url = match self.url {
            ActionInput::Static(ref url) => url.clone(),
            ActionInput::Dynamic(ref action) => {
                ctx.get_output(action.clone()).await
                    .ok_or(anyhow!("Action output not found"))?
                    .try_into()?
            }
        };

        let res = get(url).await?;
        let body = res.text().await?;

        Ok(Some(ActionOutput::String(body)))
    }

    async fn rollback(&self, _ctx: Runtime) -> Result<()> {
        Ok(())
    }

    fn display_name(&self) -> String {
        format!("HttpGet {}", match self.url {
            ActionInput::Static(ref url) => url.clone(),
            ActionInput::Dynamic(ref action) => "<dynamic>".to_owned()
        })
    }
}