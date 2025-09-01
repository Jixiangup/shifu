use crate::domain::{MavenInitOptions, Packaging};
use dialoguer::{Input, Select, theme::ColorfulTheme, Confirm};
use infra::FileWrite;
use crate::template::PomTemplate;

pub fn run(theme: &ColorfulTheme) -> Result<(), Box<dyn std::error::Error>> {
    println!("Initializing a new Maven project...");

    let packaging_variants = &[Packaging::Jar, Packaging::War, Packaging::Pom];

    // 交互获取 group_id, artifact_id, version, unstable
    let group_id: String = Input::with_theme(theme)
        .with_prompt("Group ID")
        .default("com.example".into())
        .interact_text()?;

    let artifact_id: String = Input::with_theme(theme)
        .with_prompt("Artifact ID")
        .default("my-app".into())
        .interact_text()?;

    let version: String = Input::with_theme(theme)
        .with_prompt("Version")
        .default("1.0.0".into())
        .interact_text()?;

    let packaging: usize = Select::with_theme(theme)
        .with_prompt("Select Packaging")
        .items(packaging_variants)
        .default(0)
        .interact()?;

    let unstable: bool = Confirm::with_theme(theme)
        .with_prompt("Use unstable features?")
        .default(false)
        .interact()?;

    let options = MavenInitOptions {
        group_id,
        artifact_id: artifact_id.clone(),
        version,
        packaging: packaging_variants[packaging],
        unstable,
        java_version: "17".to_string(),
        name: artifact_id,
        source_encoding: "UTF-8".to_string(),
        theme,
    };

    let template = PomTemplate::new(&options);

    template.write("pom.xml", infra::OverwritePolicy::AskIfExists)?;

    // 创建src/main/java 和 src/test/java目录

    Ok(())
}
