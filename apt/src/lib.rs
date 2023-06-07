use barley_runtime::prelude::*;
use tokio::process::Command;
use anyhow::anyhow;


pub struct AptPackage {
    name: ActionInput<String>
}

impl AptPackage {
    pub fn new<I>(name: I) -> Self
    where
        I: Into<ActionInput<String>>,
    {
        Self {
            name: name.into()
        }
    }
}

#[async_trait]
impl Action for AptPackage {
    async fn check(&self, _ctx: Runtime) -> Result<bool> {
        Ok(false)
    }

    async fn perform(&self, ctx: Runtime) -> Result<Option<ActionOutput>> {
        let name = match &self.name {
            ActionInput::Static(name) => name.clone(),
            ActionInput::Dynamic(action) => ctx.get_output(action.clone()).await
                .ok_or(anyhow!("Missing output"))?
                .try_into()?
        };

        let mut cmd = Command::new("apt-get");
        cmd.arg("install");
        cmd.arg("-y");
        cmd.arg(name);

        let output = cmd.output().await?;

        if !output.status.success() {
            return Err(anyhow!("Failed to install package"));
        }

        Ok(None)
    }

    async fn rollback(&self, _ctx: Runtime) -> Result<()> {
        Ok(())
    }

    fn display_name(&self) -> String {
        "".to_string()
    }
}

pub struct AptPackages {
    names: Vec<ActionInput<String>>
}

impl AptPackages {
    pub fn new<I, V>(names: I) -> Self
    where
        I: IntoIterator<Item = V>,
        V: Into<ActionInput<String>>,
    {
        Self {
            names: names.into_iter().map(|v| v.into()).collect()
        }
    }
}

#[async_trait]
impl Action for AptPackages {
    async fn check(&self, _ctx: Runtime) -> Result<bool> {
        Ok(false)
    }

    async fn perform(&self, ctx: Runtime) -> Result<Option<ActionOutput>> {
        let mut names = Vec::new();

        for name in &self.names {
            let name = match name {
                ActionInput::Static(name) => name.clone(),
                ActionInput::Dynamic(action) => ctx.get_output(action.clone()).await
                    .ok_or(anyhow!("Missing output"))?
                    .try_into()?
            };

            names.push(name);
        }

        let mut cmd = Command::new("apt-get");
        cmd.arg("install");
        cmd.arg("-y");
        cmd.args(names);

        let output = cmd.output().await?;

        if !output.status.success() {
            return Err(anyhow!("Failed to install packages"));
        }

        Ok(None)
    }

    async fn rollback(&self, _ctx: Runtime) -> Result<()> {
        Ok(())
    }

    fn display_name(&self) -> String {
        "".to_string()
    }
}

pub struct AptRepository {
    url: ActionInput<String>
}

impl AptRepository {
    pub fn new<I>(url: I) -> Self
    where
        I: Into<ActionInput<String>>,
    {
        Self {
            url: url.into()
        }
    }
}

#[async_trait]
impl Action for AptRepository {
    async fn check(&self, _ctx: Runtime) -> Result<bool> {
        Ok(false)
    }

    async fn perform(&self, ctx: Runtime) -> Result<Option<ActionOutput>> {
        let url = match &self.url {
            ActionInput::Static(url) => url.clone(),
            ActionInput::Dynamic(action) => ctx.get_output(action.clone()).await
                .ok_or(anyhow!("Missing output"))?
                .try_into()?
        };

        let mut cmd = Command::new("add-apt-repository");
        cmd.arg("-y");
        cmd.arg(url);

        let output = cmd.output().await?;

        if !output.status.success() {
            return Err(anyhow!("Failed to add repository"));
        }

        Ok(None)
    }

    async fn rollback(&self, _ctx: Runtime) -> Result<()> {
        Ok(())
    }

    fn display_name(&self) -> String {
        "".to_string()
    }
}

#[derive(Default)]
pub struct AptUpdate;

impl AptUpdate {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Action for AptUpdate {
    async fn check(&self, _ctx: Runtime) -> Result<bool> {
        Ok(false)
    }

    async fn perform(&self, _ctx: Runtime) -> Result<Option<ActionOutput>> {
        let mut cmd = Command::new("apt-get");
        cmd.arg("update");

        let output = cmd.output().await?;

        if !output.status.success() {
            return Err(anyhow!("Failed to update"));
        }

        Ok(None)
    }

    async fn rollback(&self, _ctx: Runtime) -> Result<()> {
        Ok(())
    }

    fn display_name(&self) -> String {
        "".to_string()
    }
}