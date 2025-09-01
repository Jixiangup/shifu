use std::error::Error;
use clap::{Parser, Subcommand};
use dialoguer::theme::ColorfulTheme;
use infra::Command;
use maven::MavenCommands;

#[derive(Parser)]
#[command(name = "shifu")]
#[command(author = "Jixon Liu")]
#[command(version = "0.1.0")]
#[command(about = "Shifu is a tool that can easily manage your daily development tasks.
 It provides a simple command-line interface that allows you to create,
 view, and manage task lists, helping you organize your work more efficiently
 and increase productivity.", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Maven {
        #[command(subcommand)]
        command: MavenCommands,
    },
}

impl Command for Commands {
    fn run(&self, theme: &ColorfulTheme) -> Result<(), Box<dyn Error>> {
        match self {
            Commands::Maven { command } => command.run(theme)?,
        }
        Ok(())
    }
}