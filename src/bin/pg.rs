#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let host = "localhost";
    let port = 4566;
    let db = "dev";
    let user = "root";
    let pass = "";

    let (client, connection) = tokio_postgres::Config::new()
        .host(host)
        .port(port)
        .dbname(db)
        .user(user)
        .password(pass)
        .connect(tokio_postgres::NoTls)
        .await?;

    let _ = tokio::spawn(async move {
        if let Err(e) = connection.await {
            // log::error!("Postgres connection error: {:?}", e);
            panic!("{}", e)
        }
    });

    let sql = "select 1;select 2;";
    let rows = client.simple_query(sql).await?;
    println!("{:#?}", rows);
    Ok(())
}
