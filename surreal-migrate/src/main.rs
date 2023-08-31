mod execute;

use std::process;

use clap::Parser;

use migrator_lib::{logger::error, model::args::Args};

use execute::execute_cmd;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!();

    let args = Args::parse();

    execute_cmd(args).await.unwrap_or_else(|err| {
        error(err);

        println!();

        process::exit(1)
    });

    println!();

    Ok(())
}
