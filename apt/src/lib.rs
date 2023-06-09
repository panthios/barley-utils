use barley_runtime::prelude::*;
use tokio::process::Command;


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
    async fn check(&self, _ctx: Runtime) -> Result<bool, ActionError> {
        Ok(false)
    }

    async fn perform(&self, ctx: Runtime) -> Result<Option<ActionOutput>, ActionError> {
        let name = match &self.name {
            ActionInput::Static(name) => name.clone(),
            ActionInput::Dynamic(action) => ctx.get_output(action.clone()).await
                .ok_or(ActionError::NoActionReturn)?
                .try_into()?
        };

        let mut cmd = Command::new("apt-get");
        cmd.arg("install");
        cmd.arg("-y");
        cmd.arg(name.clone());

        let output = cmd.output().await.map_err(|_| {
            ActionError::ActionFailed(
                format!("Failed to install package: {}", name),
                "Failed to run apt-get (internal error)".to_string()
            )
        })?;

        if !output.status.success() {
            return Err(
                ActionError::ActionFailed(
                    format!("Failed to install package: {}", name),
                    format!("-- STDERR --\n\n{}\n\n-- STDOUT --\n\n{}", String::from_utf8_lossy(&output.stderr), String::from_utf8_lossy(&output.stdout))
                )
            );
        }

        Ok(None)
    }

    async fn rollback(&self, _ctx: Runtime) -> Result<(), ActionError> {
        Ok(())
    }

    fn display_name(&self) -> String {
        format!("Install APT package {}", match self.name {
            ActionInput::Static(ref name) => name,
            ActionInput::Dynamic(_) => "<dynamic>"
        })
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
    async fn check(&self, _ctx: Runtime) -> Result<bool, ActionError> {
        Ok(false)
    }

    async fn perform(&self, ctx: Runtime) -> Result<Option<ActionOutput>, ActionError> {
        let mut names = Vec::new();

        for name in &self.names {
            let name = match name {
                ActionInput::Static(name) => name.clone(),
                ActionInput::Dynamic(action) => ctx.get_output(action.clone()).await
                    .ok_or(ActionError::NoActionReturn)?
                    .try_into()?
            };

            names.push(name);
        }

        let mut cmd = Command::new("apt-get");
        cmd.arg("install");
        cmd.arg("-y");
        cmd.args(names.clone());

        let output = cmd.output().await.map_err(|_| {
            ActionError::ActionFailed(
                format!("Failed to install packages: {:?}", names),
                "Failed to run apt-get (internal error)".to_string()
            )
        })?;

        if !output.status.success() {
            return Err(
                ActionError::ActionFailed(
                    format!("Failed to install packages: {:?}", names),
                    format!("-- STDERR --\n\n{}\n\n-- STDOUT --\n\n{}", String::from_utf8_lossy(&output.stderr), String::from_utf8_lossy(&output.stdout))
                )
            );
        }

        Ok(None)
    }

    async fn rollback(&self, _ctx: Runtime) -> Result<(), ActionError> {
        Ok(())
    }

    fn display_name(&self) -> String {
        "Install APT packages".to_string()
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
    async fn check(&self, _ctx: Runtime) -> Result<bool, ActionError> {
        Ok(false)
    }

    async fn perform(&self, ctx: Runtime) -> Result<Option<ActionOutput>, ActionError> {
        let url = match &self.url {
            ActionInput::Static(url) => url.clone(),
            ActionInput::Dynamic(action) => ctx.get_output(action.clone()).await
                .ok_or(ActionError::NoActionReturn)?
                .try_into()?
        };

        let mut cmd = Command::new("add-apt-repository");
        cmd.arg("-y");
        cmd.arg(url.clone());

        let output = cmd.output().await.map_err(|_| {
            ActionError::ActionFailed(
                format!("Failed to add repository: {}", url),
                "Failed to run add-apt-repository (internal error)".to_string()
            )
        })?;

        if !output.status.success() {
            return Err(
                ActionError::ActionFailed(
                    format!("Failed to add repository: {}", url),
                    format!("-- STDERR --\n\n{}\n\n-- STDOUT --\n\n{}", String::from_utf8_lossy(&output.stderr), String::from_utf8_lossy(&output.stdout))
                )
            );
        }

        Ok(None)
    }

    async fn rollback(&self, _ctx: Runtime) -> Result<(), ActionError> {
        Ok(())
    }

    fn display_name(&self) -> String {
        "Add repository".to_string()
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
    async fn check(&self, _ctx: Runtime) -> Result<bool, ActionError> {
        Ok(false)
    }

    async fn perform(&self, _ctx: Runtime) -> Result<Option<ActionOutput>, ActionError> {
        let mut cmd = Command::new("apt-get");
        cmd.arg("update");

        let output = cmd.output().await.map_err(|_| {
            ActionError::ActionFailed(
                "Failed to update".to_string(),
                "Failed to run apt-get (internal error)".to_string()
            )
        })?;

        if !output.status.success() {
            return Err(
                ActionError::ActionFailed(
                    "Failed to update".to_string(),
                    format!("-- STDERR --\n\n{}\n\n-- STDOUT --\n\n{}", String::from_utf8_lossy(&output.stderr), String::from_utf8_lossy(&output.stdout))
                )
            );
        }

        Ok(None)
    }

    async fn rollback(&self, _ctx: Runtime) -> Result<(), ActionError> {
        Ok(())
    }

    fn display_name(&self) -> String {
        "Update APT Cache".to_string()
    }
}