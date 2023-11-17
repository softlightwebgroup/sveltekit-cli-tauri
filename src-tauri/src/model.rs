#[derive(serde::Serialize)]
pub struct Project {
    id: String,
    name: String,
    directory: String,
}

impl Project {
    pub(crate) fn new(id: String, name: String, directory: String) -> Self {
        Self { id, name, directory }
    }
}

#[derive(serde::Serialize)]
pub struct Directory {
    name: String,
    path: String,
    is_dir: bool,
}

impl Directory {
    pub(crate) fn new(name: String, path: String, is_dir: bool) -> Self {
        Self { name, path, is_dir }
    }
}