mod args;
mod command_processor;

use {
    args::{Cli, Commands},
    clap::Parser,
};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Build { manifest_path } => {
            command_processor::run_build(manifest_path)?;
        }
    }

    Ok(())
}
