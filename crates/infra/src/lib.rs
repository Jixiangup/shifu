mod prompt;
mod write;

pub use write::*;

use dialoguer::theme::ColorfulTheme;
use std::{error::Error};

/// A trait for commands that can be run.
pub trait Command {
    fn run(&self, theme: &ColorfulTheme) -> Result<(), Box<dyn Error>>;
}