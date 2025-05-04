use rusqlite::{Connection, Error};

use crate::matching::DIR;

struct User {
    id: String,
    contact: String,
    checked: String,
}

struct Match {
    id: u32,
    user_id1: u32,
    user_id2: u32,
}

pub fn connect_db() -> Result<Connection, Box<dyn std::error::Error + Send + Sync>> {
    let conn = Connection::open(DIR.join("db.sqlite"))?;
    Ok(conn)
}

pub fn setup_db(conn: &Connection) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id          TEXT NOT NULL UNIQUE,
            contact     TEXT NOT NULL,
            checked     TEXT NOT NULL DEFAULT '[]'
        )",
        (), // empty list of parameters.
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS matches (
            id          INTEGER PRIMARY KEY,
            user_id1    INTEGER NOT NULL,
            user_id2    INTEGER NOT NULL,
            FOREIGN KEY (user_id1) REFERENCES users(id),
            FOREIGN KEY (user_id2) REFERENCES users(id)
        )",
        (),
    )?;

    Ok(())
}

fn insert_user(
    conn: &Connection,
    share_id: &str,
    contact: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if share_id.len() < 5 || share_id.len() > 8 || contact.is_empty() || contact.len() > 30 {
        return Err("Invalid share_id or contact".into());
    }

    conn.execute(
        "INSERT INTO users (id, contact) VALUES (?1, ?2)",
        (share_id, contact),
    )?;
    Ok(())
}

fn get_user(conn: &Connection, share_id: &str) -> Result<User, Box<dyn std::error::Error>> {
    let mut stmt = conn.prepare("SELECT id, share_id, contact FROM users WHERE share_id = ?1")?;
    let user = stmt.query_row([share_id], |row| {
        Ok(User {
            id: row.get(0)?,
            contact: row.get(1)?,
            checked: row.get(2)?,
        })
    })?;

    Ok(user)
}

fn insert_match(
    conn: &Connection,
    user_id: u32,
    match_id: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    conn.execute(
        "INSERT INTO matches (user_id, match_id) VALUES (?1, ?2)",
        (user_id, match_id),
    )?;
    Ok(())
}

fn get_matches(conn: &Connection, user_id: u32) -> Result<Vec<Match>, Box<dyn std::error::Error>> {
    let mut stmt = conn.prepare("SELECT id, match_id FROM matches WHERE user_id = ?1")?;
    let matches = stmt.query_map([user_id], |row| {
        Ok(Match {
            id: row.get(0)?,
            user_id1: row.get(1)?,
            user_id2: row.get(2)?,
        })
    })?;

    Ok(matches
        .into_iter()
        .map(|m| m)
        .collect::<Result<Vec<Match>, Error>>()?)
}
