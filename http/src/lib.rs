use barley_runtime::prelude::*;
use reqwest::{Client, ClientBuilder};


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
    async fn load_state(&self, builder: &mut RuntimeBuilder) {
        let client: Client = ClientBuilder::new()
            .use_rustls_tls()
            .build()
            .unwrap();

        builder.add_state(client);
    }

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

        let client: Arc<Client> = runtime.get_state().unwrap();
        let url = match &self.url {
            ActionInput::Static(url) => url.clone(),
            ActionInput::Dynamic(url) => runtime.get_output(url.clone())
                .await
                .ok_or(ActionError::NoActionReturn)?
                .try_into()?
        };

        let response = client.get(url)
            .send()
            .await
            .map_err(|e| ActionError::ActionFailed(
                format!("Failed to send request: {}", e),
                format!("Failed to send request: {}", e)
            ))?;
        
        let status = response.status();

        if !status.is_success() {
            return Err(ActionError::ActionFailed(
                format!("Request failed with status code: {}", status),
                format!("Request failed with status code: {}", status)
            ));
        }

        let body = response.text()
            .await
            .map_err(|e| ActionError::ActionFailed(
                format!("Failed to read response body: {}", e),
                format!("Failed to read response body: {}", e)
            ))?;
        
        Ok(Some(ActionOutput::String(body)))
    }

    fn display_name(&self) -> String {
        format!("HttpGet {}", match self.url {
            ActionInput::Static(ref url) => url.clone(),
            ActionInput::Dynamic(_) => "<dynamic>".to_owned()
        })
    }
}