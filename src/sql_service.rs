use itertools::Itertools;
use sql_queries::{get_essential_migrations, get_migrations};
use std::env;
use tokio::spawn;
use tokio_postgres::{Client, NoTls, connect};

pub mod queries {
    use sql_queries;

    sql_queries::load_queries!();
}

pub async fn do_migrations(client: &mut Client) -> Result<usize, tokio_postgres::Error> {
    client
        .batch_execute(&get_essential_migrations!().into_iter().join(""))
        .await
        .expect("essential migrations");

    let latest_migration: i32 = client
        .query_one_scalar(queries::GET_LATEST_MIGRATION, &[])
        .await
        .expect("latest migration");

    let mut migrated = 0;

    let tx = client.transaction().await.expect("tx");
    let stmt = tx.prepare(queries::INSERT_MIGRATION).await.expect("stmt");
    for (id, migration) in get_migrations!()
        .iter()
        .skip_while(|(id, _)| *id > latest_migration as u32)
    {
        migrated += 1;
        tx.execute(&stmt, &[id, &true]).await.expect("stmt execute");
        tx.execute(migration, &[]).await.expect("migration");
    }

    if migrated > 0 {
        tx.commit().await.expect("tx commit")
    }

    Ok(migrated)
}

pub async fn make_connection() -> Result<Client, tokio_postgres::Error> {
    let (client, connection) = connect(
        &env::var("POSTGRES_URI").expect("POSTGRES_URI env not found"),
        NoTls,
    )
    .await?;

    spawn(async move { connection.await });

    Ok(client)
}
