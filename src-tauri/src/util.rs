use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tauri::State;

use super::types::*;
use thiserror;

// create the error type that represents all errors possible in our program
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Rusqlite(#[from] rusqlite::Error),

    #[error("{0}")]
    Other(String),
}

// we must manually implement serde::Serialize
impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

use rusqlite::{params, Connection, Result};

pub fn get_active(conn: &State<Mutex<Connection>>) -> Result<usize, Error> {
    let db = conn.lock().unwrap();

    let mut exist_a = db
        .prepare("SELECT COUNT(id) FROM active")
        .expect("Exist Query Fail");
    let mut rows_exist_a = exist_a.query(()).expect("Active Run Exist Failed");
    let mut cnt_a = 0;
    while let Some(row) = rows_exist_a.next().expect("Get Active Exist Failed") {
        cnt_a = row.get(0)?;
    }

    let mut exist_p = db
        .prepare("SELECT COUNT(id) FROM projects")
        .expect("Exist Query Fail");
    let mut rows_exist_p = exist_p.query(()).expect("Active Run Exist Failed");
    let mut cnt_p = 0;
    while let Some(row) = rows_exist_p.next().expect("Get Active Exist Failed") {
        cnt_p = row.get(0)?;
    }
    if cnt_a == 0 && cnt_p != 0 {
        db.execute(
            "INSERT INTO active (project) VALUES ((SELECT min(id) FROM projects))",
            (),
        )
        .expect("Active Create Fail");
    }

    let mut qry = db
        .prepare("SELECT project FROM active")
        .expect("Active Query Fail");
    let mut rows = qry.query(()).expect("Active Run Failed");
    let mut active: usize = usize::MAX;
    while let Some(row) = rows.next().expect("Get Active Failed") {
        active = row.get(0)?;
    }
    Ok(active)
}

#[tauri::command]
pub fn get_active_cmd(conn: State<Mutex<Connection>>) -> Result<usize, Error> {
    let active: usize = get_active(&conn)?;

    Ok(active)
}

#[tauri::command]
pub fn get_project_title(conn: State<Mutex<Connection>>) -> Result<String, Error> {
    let active: usize = get_active(&conn)?;
    if active == usize::MAX {
        return Ok(String::from(""));
    }

    let db = conn.lock().unwrap();
    let mut qry = db.prepare("SELECT p.name FROM projects p WHERE p.id = ?1")?;
    let mut rows = qry.query((active,))?;

    let mut active: String = String::from("");
    while let Some(row) = rows.next()? {
        active = row.get(0)?;
    }
    Ok(active)
}
