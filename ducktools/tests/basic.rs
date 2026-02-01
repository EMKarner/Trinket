use duckdb::Connection;

#[test]
fn basic_math_still_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn duckdb_in_memory_works() {
    let conn = Connection::open_in_memory().unwrap();

    conn.execute("CREATE TABLE t(x INTEGER)", []).unwrap();
    conn.execute("INSERT INTO t VALUES (5)", []).unwrap();

    let val: i32 = conn.query_row("SELECT x FROM t", [], |r| r.get(0)).unwrap();

    assert_eq!(val, 5);
}
