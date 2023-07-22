pub mod types;

pub fn new_project(path: &str) -> types::AnimBoomProject {
    println!("New project with path: {}", path);
    types::AnimBoomProject {}
}

pub fn open_project(path: &str) -> types::AnimBoomProject {
    println!("Open project with path: {}", path);
    types::AnimBoomProject {}
}

pub fn close_project(_project: types::AnimBoomProject) {
    println!("Close project!");
}
