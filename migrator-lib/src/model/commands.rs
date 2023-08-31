use clap::Subcommand;

#[derive(Subcommand, Debug, PartialEq)]
pub enum Command {
    Init,
    Migrate {
        #[command(subcommand)]
        cmd: MigrateCmd,
    },
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum MigrateCmd {
    Add {
        #[arg(short, long, default_value_t = false)]
        down: bool,

        #[arg(short, long)]
        name: String,
    },
    Run,
    Revert,
}
