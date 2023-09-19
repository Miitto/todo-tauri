use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tauri::State;

use super::types::*;

pub fn get_active_proj(projects: &State<Projects>) -> Result<Arc<Mutex<Project>>, ()> {
    print!("{}", *projects.active.lock().unwrap());
    if projects.projects.lock().unwrap().len() == 0 {
        return Err(());
    }
    let projs = projects.projects.clone();
    let mut idx = *projects.active.lock().unwrap();
    while idx >= projs.lock().unwrap().len() && idx > 0 {
        idx -= 1;
    }
    *projects.active.lock().unwrap() = idx;
    return Ok(projects.projects.lock().unwrap()[*projects.active.lock().unwrap()].clone());
}

#[tauri::command]
pub fn get_active_proj_idx(projects: &State<Projects>) -> usize {
    if *projects.active.lock().unwrap() == usize::MAX {
        let proj = Project {
            name: String::from("New Project"),
            tasks: Vec::new(),
        };
        projects
            .projects
            .lock()
            .unwrap()
            .push(Arc::new(Mutex::new(proj)));
        *projects.active.lock().unwrap() = 0 as usize;
        println!("Made New Project");
    }
    *projects.active.lock().unwrap()
}

#[tauri::command]
pub fn get_project_title(projects: State<Projects>) -> String {
    let res = get_active_proj(&projects);
    if res.is_err() {
        return String::from("");
    }
    let proj = res.unwrap();

    let x = proj.lock().unwrap().name.clone();
    x
}
