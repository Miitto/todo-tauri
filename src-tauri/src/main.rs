// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

mod projects;
mod tasks;
mod types;
mod util;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .manage(types::Projects {
            projects: Default::default(),
            active: Mutex::new(usize::MAX),
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            util::get_project_title,
            tasks::create_task,
            tasks::get_tasks_cmd,
            tasks::delete_task,
            tasks::complete_task,
            tasks::uncomplete_task,
            projects::create_project,
            projects::get_projects_cmd,
            projects::delete_project,
            projects::get_projects_active,
            projects::set_projects_active
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
