use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Clone, Serialize, Deserialize)]
pub struct Task {
    pub name: String,
    pub done: bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub tasks: Vec<Task>,
}

pub struct Projects {
    pub projects: Arc<Mutex<Vec<Arc<Mutex<Project>>>>>,
    pub active: Mutex<usize>,
}
