// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use rusqlite::{Connection, Result};

mod projects;
mod tasks;
mod types;
mod util;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() -> Result<()> {
    let conn = Connection::open("todo.db")?;

    conn.execute(
        "create table if not exists projects (
             id integer primary key,
             name text not null unique
         )",
        (),
    )?;
    conn.execute(
        "create table if not exists tasks (
             id integer primary key,
             name text not null,
             done boolean DEFAULT false,
             project integer not null references projects(id) on delete cascade
         )",
        (),
    )?;

    conn.execute(
        "create table if not exists active (
             id integer primary key,
             project integer not null references projects(id)
         )",
        (),
    )?;

    tauri::Builder::default()
        .manage(Mutex::new(conn))
        .invoke_handler(tauri::generate_handler![
            greet,
            util::get_project_title,
            util::get_active_cmd,
            tasks::create_task,
            tasks::get_tasks_cmd,
            tasks::delete_task,
            tasks::complete_task,
            tasks::uncomplete_task,
            tasks::rename_task,
            projects::create_project,
            projects::get_projects_cmd,
            projects::delete_project,
            projects::rename_project,
            projects::set_projects_active
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
