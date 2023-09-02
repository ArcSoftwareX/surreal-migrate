use std::fs;

use crate::{
    logger::log,
    model::{commands::MigrateCmd, config::Config},
    surreal::{connect, execute_query},
    utils::get_current_timestamp,
    validator::{is_initialized, validate_name},
};

use surrealdb::{engine::remote::ws::Client, Surreal};

pub fn init() -> anyhow::Result<()> {
    if is_initialized() {
        anyhow::bail!("surreal-migrate is already initialized")
    }

    fs::create_dir_all("./migrations")?;

    fs::write(
        "./surreal-migrate.config.json",
        serde_json::to_string_pretty(&Config::default())?,
    )?;

    log("surreal-migrate initialized");

    Ok(())
}

pub async fn migrate(cmd: MigrateCmd, config: &Config) -> anyhow::Result<()> {
    if !is_initialized() {
        anyhow::bail!("surreal-migrate is not initialized")
    }

    match cmd {
        MigrateCmd::Add { down, name } => {
            let name = name.replace(' ', "_");

            if !validate_name(&name) {
                anyhow::bail!("Invalid migration name provided: {name}");
            }

            create_migration(&name, true)?;

            if down {
                create_migration(&name, false)?;
            };

            Ok(())
        }
        MigrateCmd::Run => {
            let migrations = get_migrations_up()?;

            if migrations.is_empty() {
                anyhow::bail!("No migrations to run")
            }

            let db = connect(config).await?;

            apply_migrations(db, migrations).await
        }
        MigrateCmd::Revert => {
            let migrations = get_migrations_down()?;

            if migrations.is_empty() {
                anyhow::bail!("No migrations to run")
            }

            let db = connect(config).await?;

            apply_migrations(db, migrations).await
        }
    }
}

fn get_migrations_up() -> anyhow::Result<Vec<(String, String)>> {
    let mut res = Vec::new();

    let dir = fs::read_dir("./migrations")?;

    for entry in dir {
        let entry = entry?;

        if entry.metadata()?.is_dir() {
            continue;
        }

        let path = entry.path();
        let content = fs::read_to_string(&path)?;

        let filename = entry.file_name().to_string_lossy().to_string();

        if filename.ends_with(".up.surql") {
            res.push((filename, content))
        }
    }

    Ok(res)
}

fn get_migrations_down() -> anyhow::Result<Vec<(String, String)>> {
    let mut res = Vec::new();

    let dir = fs::read_dir("./migrations")?;

    for entry in dir {
        let entry = entry?;
        let path = entry.path();
        let content = fs::read_to_string(&path)?;

        let filename = entry.file_name().to_string_lossy().to_string();

        if filename.ends_with(".down.surql") {
            res.push((filename, content))
        }
    }

    Ok(res)
}

async fn apply_migrations(
    db: Surreal<Client>,
    migrations: Vec<(String, String)>,
) -> anyhow::Result<()> {
    for migration in migrations {
        execute_query(&db, migration.1).await?;
        log(format!(
            "Executed: {}",
            migration.0.split('/').last().unwrap()
        ));
    }

    Ok(())
}

fn create_migration(name: &String, is_up: bool) -> anyhow::Result<()> {
    let timestamp = get_current_timestamp();
    let filename = format!(
        "./migrations/{}_{}.{}.surql",
        timestamp,
        name,
        if is_up { "up" } else { "down" }
    );

    log(format!(
        "Created migration: {}",
        filename.split('/').last().unwrap()
    ));

    fs::write(
        filename,
        format!(
            "# Add {} migration script here\n",
            if is_up { "up" } else { "down" }
        ),
    )?;

    Ok(())
}
