use std::{
    error::Error,
    fs,
    path::{Path, PathBuf}
};
use anyhow::anyhow;
use askama::Template;
use dialoguer::Confirm;
use infra::{FileWrite, OverwritePolicy};
use crate::domain::MavenInitOptions;

#[derive(Template)]
#[template(path = "pom.xml")]
pub struct PomTemplate<'a> {
    pub opt: &'a MavenInitOptions<'a>,
}

impl<'a> PomTemplate<'a> {
    pub fn new(opt: &'a MavenInitOptions) -> Self {
        PomTemplate { opt }
    }
}

impl<'a> FileWrite for PomTemplate<'a> {
    fn write<P: AsRef<Path>>(&self, path: P, policy: OverwritePolicy) -> Result<PathBuf, Box<dyn Error>> {
        let path = path.as_ref();
        if path.exists() {
            match policy {
                OverwritePolicy::AbortIfExists => return Err(Box::from(anyhow!("File already exists: {}", path.display()))),
                OverwritePolicy::AskIfExists => {
                    let confirm = Confirm::new()
                        .with_prompt(format!("File {} already exists. Overwrite?", path.display()))
                        .default(false)
                        .interact()?;
                    if !confirm {
                        return Err(Box::from(anyhow!("Aborted by user.")));
                    }
                }
                OverwritePolicy::RenameIfExists => {
                    let mut count = 1;
                    let new_path = loop {
                        let new_file_name = format!(
                            "{}({}){}",
                            path.file_stem().and_then(|s| s.to_str()).unwrap_or("file"),
                            count,
                            path.extension()
                                .and_then(|ext| ext.to_str())
                                .map(|ext| format!(".{}", ext))
                                .unwrap_or_default()
                        );
                        let new_path = path.with_file_name(new_file_name);
                        if !new_path.exists() {
                            break new_path;
                        }
                        count += 1;
                    };
                    return self.write(new_path, OverwritePolicy::OverwriteIfExists);
                }
                OverwritePolicy::OverwriteIfExists => {},
            }
        }
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, self.render()?)?;
        Ok(path.to_path_buf())
    }
}