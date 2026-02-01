mod db;
mod logic;

use anyhow::Result;

fn main() -> Result<()> {
    println!("🦆 DuckTools starting...");

    // open (or create) a database file inside ./data
    let conn = db::connect("data/mydb.duckdb")?;

    db::init_schema(&conn)?;

    let total = logic::example_report(&conn)?;

    println!("Report result = {total}");

    Ok(())
}
