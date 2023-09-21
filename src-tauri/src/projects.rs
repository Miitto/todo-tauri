use rusqlite::Connection;
use std::sync::Mutex;
use tauri::State;

use super::types::*;
use super::util::*;

pub fn get_projects(conn: &State<Mutex<Connection>>) -> Result<Vec<Project>, Error> {
    let db = (*conn).lock().unwrap();
    let mut qry = db.prepare("SELECT p.id, p.name FROM projects p ORDER BY p.name ASC")?;

    let proj_iter = qry.query_map((), |row| {
        Ok(Project {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    })?;

    let mut projs: Vec<Project> = Vec::new();
    for proj in proj_iter {
        projs.push(proj?);
    }

    Ok(projs)
}

#[tauri::command]
pub fn get_projects_cmd(conn: State<Mutex<Connection>>) -> Result<Vec<Project>, Error> {
    let projs = get_projects(&conn)?;
    Ok(projs)
}

#[tauri::command]
pub fn set_projects_active(
    ids: &str,
    conn: State<Mutex<Connection>>,
    window: tauri::Window,
) -> Result<usize, Error> {
    let id = ids.parse::<usize>().unwrap();
    get_active(&conn)?;
    let db = conn.lock().unwrap();
    db.execute("UPDATE active SET project = ?1", (id,))
        .expect("Update Active Failed");

    drop(db);
    let active = get_active(&conn)?;
    let _ = window.emit("active_update", active);
    Ok(active)
}

#[tauri::command]
pub fn rename_project(
    ids: &str,
    name: &str,
    conn: State<Mutex<Connection>>,
    window: tauri::Window,
) -> Result<Vec<Project>, Error> {
    let id = ids.parse::<usize>().unwrap();
    let db = conn.lock().unwrap();
    db.execute("UPDATE projects SET name = ?1 WHERE id = ?2", (name, id))
        .expect("Rename Failed");

    drop(db);
    let projs = get_projects(&conn)?;
    let _ = window.emit("project_update", projs.clone());
    Ok(projs)
}

#[tauri::command]
pub fn create_project(
    name: &str,
    conn: State<Mutex<Connection>>,
    window: tauri::Window,
) -> Result<Vec<Project>, Error> {
    let db = conn.lock().unwrap();
    db.execute("INSERT INTO projects (name) VALUES (?1)", (name,))?;

    let mut max_p = db
        .prepare("SELECT MAX(id) FROM projects")
        .expect("Exist Query Fail");
    let mut rows = max_p.query(()).expect("Get Max Id Failed");
    let mut p = 0;
    while let Some(row) = rows.next().expect("Get Active Exist Failed") {
        p = row.get(0)?;
    }

    db.execute("UPDATE active SET project = ?1", (p,))?;

    drop(rows);
    drop(max_p);
    drop(db);
    let projs = get_projects(&conn)?;
    let active = get_active(&conn)?;
    let _ = window.emit("project_update", projs.clone());
    let _ = window.emit("active_update", active);
    Ok(projs)
}

#[tauri::command]
pub fn delete_project(
    ids: &str,
    conn: State<Mutex<Connection>>,
    window: tauri::Window,
) -> Result<Vec<Project>, Error> {
    let id = ids.parse::<usize>().unwrap();

    let db = conn.lock().unwrap();

    let mut exist_p = db
        .prepare("SELECT COUNT(id) FROM projects")
        .expect("Exist Query Fail");
    let mut rows_exist_p = exist_p.query(()).expect("Active Run Exist Failed");
    let mut cnt_p = 0;
    while let Some(row) = rows_exist_p.next().expect("Get Active Exist Failed") {
        cnt_p = row.get(0)?;
    }
    println!("{:?}", cnt_p);
    if cnt_p > 1 {
        db.execute(
            "UPDATE active SET project = (SELECT min(projects.id) FROM projects WHERE projects.id != ?1)",
            (id, ),
        )?;
    } else {
        db.execute("DELETE FROM active", ())?;
    }
    db.execute("DELETE FROM projects WHERE id = ?1", (id,))?;

    drop(rows_exist_p);
    drop(exist_p);
    drop(db);

    let ts = super::tasks::get_tasks(&conn)?;
    let projs = get_projects(&conn)?;
    let _ = window.emit("task_update", ts.clone());
    let _ = window.emit("project_update", projs.clone());
    let projs = get_projects(&conn)?;
    Ok(projs)
}
