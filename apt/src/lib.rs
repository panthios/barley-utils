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
    async fn probe(&self, runtime: Runtime) -> Result<Probe, ActionError> {
        let name = match &self.name {
            ActionInput::Static(name) => name.clone(),
            ActionInput::Dynamic(action) => runtime.get_output(action.clone()).await
                .ok_or(ActionError::NoActionReturn)?
                .try_into()?
        };

        let installed = Command::new("dpkg-query")
            .arg("-W")
            .arg("-f='${Status}'")
            .arg(name.clone())
            .output()
            .await
            .map_err(|_| {
                ActionError::ActionFailed(
                    format!("Failed to check if package is installed: {}", name),
                    "Failed to run dpkg-query (internal error)".to_string()
                )
            })?
            .status
            .success();

        Ok(Probe {
            needs_run: !installed,
            can_rollback: true
        })
    }

    async fn run(&self, runtime: Runtime, op: Operation) -> Result<Option<ActionOutput>, ActionError> {
        let name = match &self.name {
            ActionInput::Static(name) => name.clone(),
            ActionInput::Dynamic(action) => runtime.get_output(action.clone()).await
                .ok_or(ActionError::NoActionReturn)?
                .try_into()?
        };

        if matches!(op, Operation::Perform) {
            let mut cmd = Command::new("apt-get");
            cmd.arg("install")
                .arg("-y")
                .arg(name.clone());

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
        } else {
            let mut cmd = Command::new("apt-get");
            cmd.arg("remove")
                .arg("-y")
                .arg(name.clone());

            let output = cmd.output().await.map_err(|_| {
                ActionError::ActionFailed(
                    format!("Failed to remove package: {}", name),
                    "Failed to run apt-get (internal error)".to_string()
                )
            })?;

            if !output.status.success() {
                return Err(
                    ActionError::ActionFailed(
                        format!("Failed to remove package: {}", name),
                        format!("-- STDERR --\n\n{}\n\n-- STDOUT --\n\n{}", String::from_utf8_lossy(&output.stderr), String::from_utf8_lossy(&output.stdout))
                    )
                );
            }

            Ok(None)
        }
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
    async fn probe(&self, runtime: Runtime) -> Result<Probe, ActionError> {
        let mut needs_run = false;

        for name in &self.names {
            let name = match name {
                ActionInput::Static(name) => name.clone(),
                ActionInput::Dynamic(action) => runtime.get_output(action.clone()).await
                    .ok_or(ActionError::NoActionReturn)?
                    .try_into()?
            };

            let installed = Command::new("dpkg-query")
                .arg("-W")
                .arg("-f='${Status}'")
                .arg(name.clone())
                .output()
                .await
                .map_err(|_| {
                    ActionError::ActionFailed(
                        format!("Failed to check if package is installed: {}", name),
                        "Failed to run dpkg-query (internal error)".to_string()
                    )
                })?
                .status
                .success();

            if !installed {
                needs_run = true;
                break;
            }
        }

        Ok(Probe {
            needs_run,
            can_rollback: true
        })
    }

    async fn run(&self, runtime: Runtime, op: Operation) -> Result<Option<ActionOutput>, ActionError> {
        if matches!(op, Operation::Perform) {
            let mut cmd = Command::new("apt-get");
            cmd.arg("install")
                .arg("-y");

            for name in &self.names {
                let name = match name {
                    ActionInput::Static(name) => name.clone(),
                    ActionInput::Dynamic(action) => runtime.get_output(action.clone()).await
                        .ok_or(ActionError::NoActionReturn)?
                        .try_into()?
                };

                cmd.arg(name);
            }

            let output = cmd.output().await.map_err(|_| {
                ActionError::ActionFailed(
                    "Failed to install packages".to_string(),
                    "Failed to run apt-get (internal error)".to_string()
                )
            })?;

            if !output.status.success() {
                return Err(
                    ActionError::ActionFailed(
                        "Failed to install packages".to_string(),
                        format!("-- STDERR --\n\n{}\n\n-- STDOUT --\n\n{}", String::from_utf8_lossy(&output.stderr), String::from_utf8_lossy(&output.stdout))
                    )
                );
            }

            Ok(None)
        } else {
            let mut cmd = Command::new("apt-get");
            cmd.arg("remove")
                .arg("-y");

            for name in &self.names {
                let name = match name {
                    ActionInput::Static(name) => name.clone(),
                    ActionInput::Dynamic(action) => runtime.get_output(action.clone()).await
                        .ok_or(ActionError::NoActionReturn)?
                        .try_into()?
                };

                cmd.arg(name);
            }

            let output = cmd.output().await.map_err(|_| {
                ActionError::ActionFailed(
                    "Failed to remove packages".to_string(),
                    "Failed to run apt-get (internal error)".to_string()
                )
            })?;

            if !output.status.success() {
                return Err(
                    ActionError::ActionFailed(
                        "Failed to remove packages".to_string(),
                        format!("-- STDERR --\n\n{}\n\n-- STDOUT --\n\n{}", String::from_utf8_lossy(&output.stderr), String::from_utf8_lossy(&output.stdout))
                    )
                );
            }

            Ok(None)
        }
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
    async fn probe(&self, runtime: Runtime) -> Result<Probe, ActionError> {
        let url = match &self.url {
            ActionInput::Static(url) => url.clone(),
            ActionInput::Dynamic(action) => runtime.get_output(action.clone()).await
                .ok_or(ActionError::NoActionReturn)?
                .try_into()?
        };

        let mut cmd = Command::new("apt-cache");
        cmd.arg("policy");
        cmd.arg(url.clone());

        let output = cmd.output().await.map_err(|_| {
            ActionError::ActionFailed(
                format!("Failed to check if repository is installed: {}", url),
                "Failed to run apt-cache (internal error)".to_string()
            )
        })?;

        let installed = output.status.success();

        Ok(Probe {
            needs_run: !installed,
            can_rollback: true
        })
    }

    async fn run(&self, runtime: Runtime, op: Operation) -> Result<Option<ActionOutput>, ActionError> {
        let url = match &self.url {
            ActionInput::Static(url) => url.clone(),
            ActionInput::Dynamic(action) => runtime.get_output(action.clone()).await
                .ok_or(ActionError::NoActionReturn)?
                .try_into()?
        };

        if matches!(op, Operation::Perform) {
            let mut cmd = Command::new("add-apt-repository");
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
        } else {
            let mut cmd = Command::new("add-apt-repository");
            cmd.arg("-r");
            cmd.arg(url.clone());

            let output = cmd.output().await.map_err(|_| {
                ActionError::ActionFailed(
                    format!("Failed to remove repository: {}", url),
                    "Failed to run add-apt-repository (internal error)".to_string()
                )
            })?;

            if !output.status.success() {
                return Err(
                    ActionError::ActionFailed(
                        format!("Failed to remove repository: {}", url),
                        format!("-- STDERR --\n\n{}\n\n-- STDOUT --\n\n{}", String::from_utf8_lossy(&output.stderr), String::from_utf8_lossy(&output.stdout))
                    )
                );
            }

            Ok(None)
        }
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
    async fn probe(&self, _runtime: Runtime) -> Result<Probe, ActionError> {
        Ok(Probe {
            needs_run: true,
            can_rollback: false
        })
    }

    async fn run(&self, _runtime: Runtime, op: Operation) -> Result<Option<ActionOutput>, ActionError> {
        if matches!(op, Operation::Rollback) {
            return Err(ActionError::OperationNotSupported);
        }

        let mut cmd = Command::new("apt-get");
        cmd.arg("update");

        let output = cmd.output().await.map_err(|_| {
            ActionError::ActionFailed(
                "Failed to update APT cache".to_string(),
                "Failed to run apt-get (internal error)".to_string()
            )
        })?;

        if !output.status.success() {
            return Err(
                ActionError::ActionFailed(
                    "Failed to update APT cache".to_string(),
                    format!("-- STDERR --\n\n{}\n\n-- STDOUT --\n\n{}", String::from_utf8_lossy(&output.stderr), String::from_utf8_lossy(&output.stdout))
                )
            );
        }

        Ok(None)
    }

    fn display_name(&self) -> String {
        "Update APT Cache".to_string()
    }
}