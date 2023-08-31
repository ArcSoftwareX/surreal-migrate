use std::process;

use migrator_lib::{
    logger::error,
    migrator::{init, migrate},
    model::{args::Args, commands::Command, config::Config},
};

pub async fn execute_cmd(args: Args) -> anyhow::Result<()> {
    let config = if args.cmd != Command::Init {
        Some(Config::new(&args).unwrap_or_else(|err| {
            error(err);
            process::exit(1)
        }))
    } else {
        None
    };

    match args.cmd {
        Command::Init => init(),
        Command::Migrate { cmd } => migrate(cmd, &config.unwrap()).await,
    }
}
