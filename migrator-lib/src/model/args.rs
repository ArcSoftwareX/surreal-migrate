use clap::Parser;

use super::commands::Command;

#[derive(Parser)]
#[clap(
    author = "ArcSoftwareX",
    version,
    name = "surreal-migrate",
    about = "A simple migration tool for SurrealDB"
)]
pub struct Args {
    #[arg(short, long, default_value = "./surreal-migrate.config.json")]
    pub config_url: String,

    #[command(subcommand)]
    pub cmd: Command,
}
