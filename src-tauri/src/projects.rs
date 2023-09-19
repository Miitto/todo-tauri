use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tauri::State;

use super::types::*;

pub fn get_projects(projects: State<Projects>) -> Vec<Project> {
    let proj = projects.projects.lock().unwrap().clone();
    let mut projs = Vec::new();
    for p in proj {
        projs.push(p.lock().unwrap().clone())
    }
    projs
}

#[tauri::command]
pub fn get_projects_cmd(projects: State<Projects>) -> Vec<Project> {
    let projs = get_projects(projects);
    projs
}

#[tauri::command]
pub fn get_projects_active(projects: State<Projects>) -> usize {
    let active = projects.active.lock().unwrap().clone();
    active
}

#[tauri::command]
pub fn set_projects_active(ids: &str, projects: State<Projects>, window: tauri::Window) {
    let id = ids.parse::<usize>().unwrap();
    *projects.active.lock().unwrap() = id;

    let active = projects.active.lock().unwrap().clone();
    let _ = window.emit("active_update", active);
}

#[tauri::command]
pub fn create_project(
    name: &str,
    projects: State<Projects>,
    window: tauri::Window,
) -> Vec<Project> {
    projects
        .projects
        .lock()
        .unwrap()
        .push(Arc::new(Mutex::new(Project {
            name: String::from(name),
            tasks: Vec::new(),
        })));

    *projects.active.lock().unwrap() = projects.projects.lock().unwrap().len() - 1;
    let active = projects.active.lock().unwrap().clone();

    let projs = get_projects(projects);

    let _ = window.emit("project_update", projs.clone());
    let _ = window.emit("active_update", active);
    projs
}

#[tauri::command]
pub fn delete_project(ids: &str, projects: State<Projects>, window: tauri::Window) {
    let id = ids.parse::<usize>().unwrap();

    let active = projects.active.lock().unwrap().clone();

    if id < projects.projects.lock().unwrap().len() {
        projects.projects.lock().unwrap().remove(id);
    }

    if active >= id && id > 0 {
        *projects.active.lock().unwrap() -= 1;
        let _ = window.emit("active_update", active);
    } else if id < 1 {
        *projects.active.lock().unwrap() = usize::MAX;
    }

    let projs = get_projects(projects);

    let _ = window.emit("project_update", projs.clone());
}
