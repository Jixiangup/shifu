use std::fmt::{Display, Formatter};
use dialoguer::theme::ColorfulTheme;

pub struct ClassOptions {
    pub author: String,
}

pub struct MavenInitOptions<'a> {
    pub theme: &'a ColorfulTheme,
    pub group_id: String,
    pub artifact_id: String,
    pub version: String,
    pub packaging: Packaging,
    pub unstable: bool,
    pub java_version: String,
    pub name: String,
    pub source_encoding: String,
}

#[derive(Clone, Debug, Copy)]
pub enum Packaging {
    Jar,
    War,
    Pom,
}

impl Default for Packaging {
    fn default() -> Self {
        Packaging::Jar
    }
}

impl Display for Packaging {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Packaging::Jar => "jar",
            Packaging::War => "war",
            Packaging::Pom => "pom",
        };
        write!(f, "{}", s)
    }
}

impl From<String> for Packaging {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "jar" => Packaging::Jar,
            "war" => Packaging::War,
            "pom" => Packaging::Pom,
            _ => Packaging::Jar, // default to Jar if unknown
        }
    }
}