use rusqlite::{Connection, Error};
use serde::Deserialize;
use serde_json::from_str;

use crate::matching::DIR;

#[derive(Debug)]
pub struct User {
    pub id: String,
    pub twitter_handle: String,
    pub checked: Vec<String>,
    pub hash: String,
}

#[derive(Debug)]
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
            id                  TEXT NOT NULL UNIQUE,
            twitter_handle      TEXT NOT NULL,
            checked             TEXT NOT NULL DEFAULT '[]',
            hash                TEXT NOT NULL
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

pub fn insert_user(
    conn: &Connection,
    id: &str,
    twitter_handle: &str,
    hash: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if twitter_handle.is_empty() || twitter_handle.len() > 30 {
        return Err("Invalid twitter handle".into());
    }

    let checked = serde_json::to_string(&vec![id])?;

    conn.execute(
        "INSERT INTO users (id, twitter_handle, checked, hash) VALUES (?1, ?2, ?3, ?4)",
        (id, twitter_handle, checked, hash),
    )?;

    Ok(())
}

pub fn get_user(
    conn: &Connection,
    user_id: &str,
) -> Result<User, Box<dyn std::error::Error + Send + Sync>> {
    let mut stmt = conn.prepare("SELECT * FROM users WHERE id = ?1")?;
    let user = stmt.query_row([user_id], |row| {
        let checked: String = row.get(2)?;
        Ok(User {
            id: row.get(0)?,
            twitter_handle: row.get(1)?,
            checked: serde_json::from_str(&checked).map_err(|_| rusqlite::Error::InvalidQuery)?,
            hash: row.get(3)?,
        })
    })?;

    Ok(user)
}

pub fn get_all_users(
    conn: &Connection,
) -> Result<Vec<User>, Box<dyn std::error::Error + Send + Sync>> {
    let mut stmt = conn.prepare("SELECT * FROM users")?;
    let users = stmt.query_map([], |row| {
        let checked: String = row.get(2)?;
        Ok(User {
            id: row.get(0)?,
            twitter_handle: row.get(1)?,
            checked: serde_json::from_str(&checked).map_err(|_| rusqlite::Error::InvalidQuery)?,
            hash: row.get(3)?,
        })
    })?;

    Ok(users.collect::<Result<Vec<User>, Error>>()?)
}

pub fn update_checked(
    conn: &Connection,
    user_id: &str,
    new_checked: Vec<String>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let user = get_user(conn, user_id)?;

    let mut checked = user.checked;
    checked.extend(new_checked);

    conn.execute(
        "UPDATE users SET checked = ?1 WHERE id = ?2",
        (serde_json::to_string(&checked)?, user_id),
    )?;
    Ok(())
}

fn insert_match(
    conn: &Connection,
    user_id1: u32,
    user_id2: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    conn.execute(
        "INSERT INTO matches (user_id1, user_id2) VALUES (?1, ?2)",
        (user_id1, user_id2),
    )?;
    Ok(())
}

fn get_matches(conn: &Connection, user_id: u32) -> Result<Vec<Match>, Box<dyn std::error::Error>> {
    let mut stmt = conn.prepare("SELECT * FROM matches WHERE (user_id1 = ?1 OR user_id2 = ?1)")?;
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

pub fn does_hash_exist(
    conn: &Connection,
    hash: &str,
) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM users WHERE hash = ?1")?;
    let count: i32 = stmt.query_row([hash], |row| row.get(0))?;
    Ok(count > 0)
}
