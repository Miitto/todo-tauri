use super::types::*;
use super::util::*;

use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tauri::State;

pub fn get_tasks(project: Project) -> Vec<Task> {
    project.tasks.to_vec()
}

#[tauri::command]
pub fn get_tasks_cmd(projects: State<Projects>) -> Vec<Task> {
    let res = get_active_proj(&projects);
    if res.is_err() {
        return Vec::new();
    }
    let proj = res.unwrap();

    let tasks = get_tasks(proj.lock().unwrap().clone());
    tasks
}

#[tauri::command]
pub fn create_task(name: &str, projects: State<Projects>, window: tauri::Window) -> Vec<Task> {
    let res = get_active_proj(&projects);
    if res.is_err() {
        return Vec::new();
    }
    let proj = res.unwrap();

    proj.lock().unwrap().tasks.push(Task {
        name: String::from(name),
        done: false,
    });

    let tasks = get_tasks(proj.lock().unwrap().clone());
    let _ = window.emit("task_update", tasks.to_owned());
    tasks
}

#[tauri::command]
pub fn delete_task(id: &str, projects: State<Projects>, window: tauri::Window) {
    let res = get_active_proj(&projects);
    if res.is_err() {
        return;
    }
    let proj = res.unwrap();

    if id.parse::<usize>().unwrap() < proj.lock().unwrap().tasks.len() {
        proj.lock()
            .unwrap()
            .tasks
            .remove(id.parse::<usize>().unwrap());
    }

    let _ = window.emit("task_update", proj.lock().unwrap().tasks.to_owned());
}

#[tauri::command]
pub fn complete_task(id: &str, projects: State<Projects>) {
    let res = get_active_proj(&projects);
    if res.is_err() {
        return;
    }
    let proj = res.unwrap();

    if id.parse::<usize>().unwrap() < proj.lock().unwrap().tasks.len() {
        proj.lock().unwrap().tasks[id.parse::<usize>().unwrap()].done = true;
    }
}

#[tauri::command]
pub fn uncomplete_task(id: &str, projects: State<Projects>) {
    let res = get_active_proj(&projects);
    if res.is_err() {
        return;
    }
    let proj = res.unwrap();

    if id.parse::<usize>().unwrap() < proj.lock().unwrap().tasks.len() {
        proj.lock().unwrap().tasks[id.parse::<usize>().unwrap()].done = false;
    }
}
