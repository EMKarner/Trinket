use duckdb::Connection;
use duckdb::Result;

use crate::db;

pub fn example_report(conn: &Connection) -> Result<i32> {
    let total = db::sum_sales(conn)?;

    // business logic lives here
    // pretend we apply some finance wizardry
    let adjusted = total * 2;

    Ok(adjusted)
}
