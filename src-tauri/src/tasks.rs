use super::types::*;
use super::util::*;

use rusqlite::Connection;
use std::sync::Mutex;
use tauri::State;

pub fn get_tasks(conn: &State<Mutex<Connection>>) -> Result<Vec<Task>, Error> {
    let active: usize = get_active(conn)?;
    if active == usize::MAX {
        return Ok(Vec::new());
    }

    let db = conn.lock().unwrap();
    let mut qry =
        db.prepare("SELECT t.id, t.name, t.done FROM projects p, tasks t WHERE p.id = :id AND t.project = p.id")?;

    let task_iter = qry.query_map(&[(":id", &active)], |row| {
        Ok(Task {
            id: row.get(0)?,
            name: row.get(1)?,
            done: row.get(2)?,
        })
    })?;

    let mut ts: Vec<Task> = Vec::new();
    for task in task_iter {
        ts.push(task.unwrap());
    }

    Ok(ts)
}

#[tauri::command]
pub fn get_tasks_cmd(conn: State<Mutex<Connection>>) -> Result<Vec<Task>, Error> {
    let ts = get_tasks(&conn)?;
    Ok(ts)
}

#[tauri::command]
pub fn create_task(
    name: &str,
    conn: State<Mutex<Connection>>,
    window: tauri::Window,
) -> Result<Vec<Task>, Error> {
    let active: usize = get_active(&conn)?;
    if active == usize::MAX {
        return Ok(Vec::new());
    }

    let db = conn.lock().unwrap();
    db.execute(
        "INSERT INTO tasks (name, project) VALUES (?1, ?2)",
        (name, &active),
    )?;

    drop(db);
    let ts = get_tasks(&conn)?;

    let _ = window.emit("task_update", ts.clone());
    Ok(ts)
}

#[tauri::command]
pub fn delete_task(
    id: &str,
    conn: State<Mutex<Connection>>,
    window: tauri::Window,
) -> Result<Vec<Task>, Error> {
    let db = conn.lock().unwrap();
    db.execute("DELETE FROM tasks WHERE id = ?1", &[id])?;

    drop(db);
    let ts = get_tasks(&conn)?;
    let _ = window.emit("task_update", ts.clone());
    Ok(ts)
}

#[tauri::command]
pub fn complete_task(
    id: &str,
    conn: State<Mutex<Connection>>,
    window: tauri::Window,
) -> Result<Vec<Task>, Error> {
    let db = conn.lock().unwrap();
    db.execute("UPDATE tasks SET done = true WHERE id = ?1", &[id])?;

    drop(db);
    let ts = get_tasks(&conn)?;
    let _ = window.emit("task_update", ts.clone());
    Ok(ts)
}

#[tauri::command]
pub fn uncomplete_task(
    id: &str,
    conn: State<Mutex<Connection>>,
    window: tauri::Window,
) -> Result<Vec<Task>, Error> {
    let db = conn.lock().unwrap();
    db.execute("UPDATE tasks SET done = false WHERE id = ?1", &[id])?;

    drop(db);
    let ts = get_tasks(&conn)?;
    let _ = window.emit("task_update", ts.clone());
    Ok(ts)
}

#[tauri::command]
pub fn rename_task(
    ids: &str,
    name: &str,
    conn: State<Mutex<Connection>>,
    window: tauri::Window,
) -> Result<Vec<Task>, Error> {
    let db = conn.lock().unwrap();
    db.execute("UPDATE tasks SET name = ?1 WHERE id = ?2", &[name, ids])?;

    drop(db);
    let ts = get_tasks(&conn)?;
    let _ = window.emit("task_update", ts.clone());
    Ok(ts)
}
