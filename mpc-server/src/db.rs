use rusqlite::{Connection, Error, params_from_iter};
use std::collections::HashSet;

use crate::matching::DATA_DIR;

#[derive(Debug, Clone)]
pub struct User {
    pub id: String,
    pub twitter_handle: String,
    pub checked: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Match {
    pub id: u32,
    pub user_id1: String,
    pub user_id2: String,
}

pub fn connect_db() -> Result<Connection, Box<dyn std::error::Error + Send + Sync>> {
    let conn = Connection::open(DATA_DIR.join("db.sqlite"))?;
    Ok(conn)
}

pub fn setup_db() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let conn = connect_db()?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id                  TEXT NOT NULL UNIQUE,
            twitter_handle      TEXT NOT NULL,
            checked             TEXT NOT NULL DEFAULT '[]'
        )",
        (), // empty list of parameters.
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS matches (
            id          INTEGER PRIMARY KEY,
            user_id1    TEXT NOT NULL,
            user_id2    TEXT NOT NULL,
            FOREIGN KEY (user_id1) REFERENCES users(id),
            FOREIGN KEY (user_id2) REFERENCES users(id),
            UNIQUE(user_id1, user_id2)
        )",
        (),
    )?;

    Ok(())
}

pub fn insert_user(
    conn: &Connection,
    id: &str,
    twitter_handle: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if twitter_handle.is_empty() || twitter_handle.len() > 30 {
        return Err("Invalid twitter handle".into());
    }

    let checked = serde_json::to_string(&vec![id])?;

    conn.execute(
        "INSERT INTO users (id, twitter_handle, checked) VALUES (?1, ?2, ?3)",
        (id, twitter_handle, checked),
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

    let mut checked: HashSet<String> = user.checked.into_iter().collect();
    checked.extend(new_checked);
    println!("checked: {:?}", checked);

    conn.execute(
        "UPDATE users SET checked = ?1 WHERE id = ?2",
        (serde_json::to_string(&checked)?, user_id),
    )?;
    Ok(())
}

pub fn update_checked_many(
    conn: &Connection,
    user_ids: Vec<String>,
    new_checked: Vec<String>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut stmt = conn.prepare("UPDATE users SET checked = ?1 WHERE id = ?2")?;

    for user_id in user_ids {
        let user = get_user(conn, &user_id)?;
        let mut checked: HashSet<String> = user.checked.into_iter().collect();
        checked.extend(new_checked.clone());
        stmt.execute((serde_json::to_string(&checked)?, user_id))?;
    }
    Ok(())
}

pub fn insert_matches(
    conn: &Connection,
    matches: Vec<(String, String)>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut stmt = conn.prepare("INSERT INTO matches (user_id1, user_id2) VALUES (?1, ?2)")?;
    for (user_id1, user_id2) in matches {
        stmt.execute((user_id1, user_id2))?;
    }
    Ok(())
}

pub fn get_matches(
    user_id: String,
) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
    let conn = connect_db()?;
    let mut stmt = conn.prepare("SELECT * FROM matches WHERE (user_id1 = ?1 OR user_id2 = ?1)")?;
    let matches = stmt.query_map([user_id.clone()], |row| {
        Ok(Match {
            id: row.get(0)?,
            user_id1: row.get(1)?,
            user_id2: row.get(2)?,
        })
    })?;

    let matches = matches
        .into_iter()
        .map(|m| m)
        .collect::<Result<Vec<Match>, Error>>()?
        .iter()
        .map(|m| {
            if m.user_id1 == user_id.clone() {
                m.user_id2.clone()
            } else {
                m.user_id1.clone()
            }
        })
        .collect::<Vec<String>>();

    if matches.is_empty() {
        return Ok(vec![]);
    }

    fn repeat_vars(count: usize) -> String {
        assert_ne!(count, 0);
        let mut s = "?,".repeat(count);
        // Remove trailing comma
        s.pop();
        s
    }

    let mut stmt = conn.prepare(&format!(
        "SELECT twitter_handle FROM users WHERE id IN ({})",
        repeat_vars(matches.iter().count())
    ))?;
    let user_matches = stmt.query_map(params_from_iter(matches.iter()), |row| row.get(0))?;
    let user_matches = user_matches.collect::<Result<Vec<String>, Error>>()?;

    Ok(user_matches)
}
