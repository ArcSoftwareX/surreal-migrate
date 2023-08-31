use crate::model::config::Config;
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

pub async fn connect(config: &Config) -> anyhow::Result<Surreal<Client>> {
    let db = Surreal::new::<Ws>(config.database_url.clone()).await?;

    db.signin(Root {
        username: &config.user,
        password: &config.pass,
    })
    .await?;

    db.use_ns(config.ns.clone())
        .use_db(config.db.clone())
        .await?;

    Ok(db)
}

pub async fn execute_query(db: &Surreal<Client>, query: String) -> anyhow::Result<()> {
    db.query(query).await?;

    Ok(())
}
