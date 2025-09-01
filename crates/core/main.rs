use clap::Parser;
use cli::Cli;
use dialoguer::theme::ColorfulTheme;
use infra::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    cli.command.run(&ColorfulTheme::default())?;
    Ok(())
}
