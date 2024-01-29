use anyhow::{anyhow, Result};
use scylla::transport::session::Session;
use scylla::SessionBuilder;


#[tokio::main]
async fn main() -> Result<()> {

    let uri = "localhost:9042";

    println!("Connecting to {} ...", uri);

    let session: Session = SessionBuilder::new().known_node(uri).user("", "").build().await?;

    let query_result = session
        .query("SELECT user_id FROM kstltest.users", &[])
        .await?;
    let (usr_id_idx, _) = query_result
        .get_column_spec("user_id")
        .ok_or_else(|| anyhow!("No ck column found"))?;

    println!("---------------------");
    for row in query_result.rows.ok_or_else(|| anyhow!("no rows found"))? {
        // user_id column is a text column (user_id text) (see: https://github.com/KastelApp/CqlTables/blob/master/Tables/UserSchema.cql#L2)
        let user_id = row.columns[usr_id_idx]
            .as_ref()
            .unwrap()
            .as_text()
            .unwrap();

        println!("user_id: {}", user_id);
    }

    Ok(())
}