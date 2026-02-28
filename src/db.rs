use rusqlite::{Connection, params};

use crate::status::Status;

pub fn make_connection(location: &str) -> anyhow::Result<Connection> {
    let conn = Connection::open(location)?;
    
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS state (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            humidity REAL,
            temperature REAL,
            mainroom_status INTEGER,
            bathroom_status INTEGER
        );

        INSERT OR IGNORE INTO state (id) VALUES (1);
        "#
    )?;

    Ok(conn)
}

pub fn get_status(conn: &Connection) -> anyhow::Result<Status> {
    let status = conn.query_row(
        r#"
        SELECT humidity, temperature, mainroom_status, bathroom_status
        FROM state WHERE id = 1
        "#,
        [],
        |row| {
            Ok(Status {
                humidity: row.get(0)?,
                temprature: row.get(1)?,
                mainroom_status: row.get::<_, Option<i64>>(2)?.map(|v| v != 0),
                bathroom_status: row.get::<_, Option<i64>>(3)?.map(|v| v != 0),
            })
        },
    )?;

    Ok(status)
}

pub fn save_state(conn: &Connection, state: &Status) -> anyhow::Result<()> {
    conn.execute(
        r#"
        UPDATE state SET
            humidity = ?1,
            temperature = ?2,
            mainroom_status = ?3,
            bathroom_status = ?4
        WHERE id = 1
        "#,
        params![
            state.humidity,
            state.temprature,
            state.mainroom_status.map(|v| v as i64),
            state.bathroom_status.map(|v| v as i64),
        ],
    )?;
    Ok(())
}