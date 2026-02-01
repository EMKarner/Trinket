use duckdb::{Connection, Result};

pub fn connect(path: &str) -> Result<Connection> {
    Connection::open(path)
}

pub fn init_schema(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS sales (
            id INTEGER,
            amount INTEGER
        );
        ",
    )?;

    // seed some demo data
    conn.execute("INSERT INTO sales VALUES (1, 10)", [])?;
    conn.execute("INSERT INTO sales VALUES (2, 25)", [])?;

    Ok(())
}

pub fn sum_sales(conn: &Connection) -> Result<i32> {
    let mut stmt = conn.prepare("SELECT SUM(amount) FROM sales")?;
    let mut rows = stmt.query([])?;

    if let Some(row) = rows.next()? {
        let val: i32 = row.get(0)?;
        Ok(val)
    } else {
        Ok(0)
    }
}
