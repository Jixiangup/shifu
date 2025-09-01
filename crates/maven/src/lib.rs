mod init;
mod domain;
mod template;

use std::error::Error;
use clap::Subcommand;
use dialoguer::theme::ColorfulTheme;
use infra::Command;

#[derive(Subcommand)]
pub enum MavenCommands {
    // 在目录下面初始化一个maven项目
    Init,
}

/// A struct representing a Maven project.
pub struct Maven {
    /// The group ID of the Maven project.
    /// For example, "com.example".
    /// This is typically a reversed domain name that you control.
    /// It helps to uniquely identify your project across all projects.
    /// It is a required field in the `pom.xml` file.
    /// It is used to organize and manage your project's dependencies and artifacts.
    /// It is also used in the package structure of your Java classes.
    /// Example: "com.example"
    /// See: https://maven.apache.org/guides/mini/guide-naming-conventions.html
    pub group_id: String,

    /// The artifact ID of the Maven project.
    /// For example, "my-app".
    /// This is the name of the project and is used to identify the project within a group.
    /// It is a required field in the `pom.xml` file.
    /// It is used in the naming of the generated artifact (e.g., JAR file).
    /// It is also used in the URL of the project's repository.
    /// Example: "my-app"
    /// See: https://maven.apache.org/guides/mini/guide-naming_conventions.html
    pub artifact_id: String,

    /// The version of the Maven project.
    /// For example, "1.0.0".
    /// This is the version of the project and is used to identify different releases of the project.
    /// It is a required field in the `pom.xml` file.
    /// It is used in the naming of the generated artifact (e.g., JAR file).
    /// It is also used in the URL of the project's repository.
    /// Example: "1.0.0"
    /// See: https://maven.apache.org/guides/mini/guide-naming_conventions.html
    pub version: String,

    /// Whether to use unstable features in the Maven project.
    /// This is an optional field and defaults to `false`.
    /// If set to `true`, it enables the use of unstable features that may not be
    /// fully tested or supported.
    /// This can be useful for experimenting with new features or for development purposes.
    /// However, it may also introduce instability or compatibility issues.
    /// Use with caution and only if you understand the risks involved.
    /// Default: false
    /// Example: true
    /// See: https://maven.apache.org/guides/mini/guide-unstable-features.html
    pub unstable: bool,
}

impl Command for MavenCommands {
    fn run(&self, theme: &ColorfulTheme) -> Result<(), Box<dyn Error>> {
        match self {
            MavenCommands::Init => {
                init::run(theme)?;
            }
        }
        Ok(())
    }
}