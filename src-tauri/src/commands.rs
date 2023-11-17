use fs::read_to_string;
use std::fs;
use std::io::BufRead;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use convert_case::{Case, Casing};
use tauri::{command, State, Window};

use crate::db::DatabaseConnection;
use crate::functions::fn_get_routes;
use crate::model::{Directory, Project};

#[cfg(windows)]
pub const NPM: &'static str = "npm.cmd";

#[cfg(not(windows))]
pub const NPM: &'static str = "npm";


#[command]
pub fn get_projects(state: State<'_, DatabaseConnection>) -> Vec<Project> {
    let query = "select id, name, directory from projects order by created_at desc";

    let mut projects = Vec::new();

    let conn = state.conn.lock().unwrap();
    let mut stmt = conn.prepare(query).unwrap();

    let projects_iter = stmt.query_map([], |row| {
        Ok(Project::new(
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
        ))
    }).unwrap();

    for project in projects_iter {
        projects.push(project.unwrap())
    }

    projects
}

#[command]
pub fn save_project(state: State<'_, DatabaseConnection>, id: String, name: String, directory: String) {
    let query = "insert into projects (id, name, directory) values (?, ?, ?)";

    let conn = state.conn.lock().unwrap();

    conn.execute(query, &[&id, &name, &directory]).unwrap();

    println!("Saved project {} with directory {}", name, directory);
}

#[command]
pub fn get_files(state: State<'_, DatabaseConnection>, project_id: String, path: String) -> Vec<Directory> {
    let paths = fs::read_dir(path).unwrap();

    let mut directory = Vec::new();

    for path in paths {
        let path = path.unwrap().path();
        let option = path.file_name().unwrap().to_str().unwrap();

        directory.push(Directory::new(option.to_string(), path.display().to_string(), path.is_dir()))
    }

    directory
}

#[command]
pub fn get_commands(state: State<'_, DatabaseConnection>, path: String) -> Vec<String> {
    let file = format!("{}/package.json", path);
    let file_content = fs::read(file).unwrap();

    let package = serde_json::from_slice::<serde_json::Value>(&file_content).unwrap();

    let commands = package
        .as_object()
        .unwrap()
        .get("scripts")
        .unwrap()
        .as_object()
        .unwrap()
        .keys()
        .map(|key| key.to_string())
        .collect::<Vec<String>>();

    commands
}

#[command]
pub fn run_command(window: Window, state: State<'_, DatabaseConnection>, path: String, command: String) {
    let mut child = Command::new(NPM)
        .arg("run")
        .arg(command)
        .current_dir(path)
        .stdout(Stdio::piped()) // Configura la salida estándar para ser capturada
        .spawn()
        .expect("falló la ejecución del comando");

    if let Some(ref mut stdout) = child.stdout {
        let reader = std::io::BufReader::new(stdout);
        for line in reader.lines() {
            if let Ok(line) = line {
                window.emit("command:output", &line).expect("error al emitir el evento");
            }
        }
    }
}

#[command]
pub fn get_routes(state: State<'_, DatabaseConnection>, path: String) -> Vec<String> {
    fn_get_routes(path)
}

#[command]
pub fn create_route(state: State<'_, DatabaseConnection>, route: String, directory: String, pages: Vec<String>) -> Vec<String> {
    let route_clean = route.replace(" ", "-");
    let mut directory_path = PathBuf::from(&directory);
    directory_path.push("src/routes");
    directory_path.push(&route_clean);

    println!("DIR: {:?}", directory_path);

    if !directory_path.exists() {
        println!("CREATING DIR: {:?}", directory_path);
        if let Err(e) = fs::create_dir_all(&directory_path) {
            eprintln!("Error al crear el directorio: {}", e);
            return Vec::new();
        }
    }

    for page in pages {
        let mut file_path = directory_path.clone();
        file_path.push(&page);
        println!("PATH: {:?}", file_path);

        if file_path.exists() {
            continue;
        }

        match read_to_string(format!("./src/templates/{page}")) {
            Ok(content) => {
                if let Err(e) = fs::write(&file_path, content) {
                    eprintln!("Error al escribir en el archivo: {}", e);
                    return Vec::new();
                }
            }
            Err(e) => {
                eprintln!("Error al leer el archivo: {} - {}", e, page);
                return Vec::new();
            }
        }
    }

    fn_get_routes(directory)
}

#[command]
pub fn create_component(state: State<'_, DatabaseConnection>, component: String, directory: String) -> Vec<String> {
    //convert component to camel case and remove spaces
    let component = component.to_case(Case::UpperCamel);
    let mut directory_path = PathBuf::from(&directory);
    directory_path.push("src/lib/components");


    if !directory_path.exists() {
        if let Err(e) = fs::create_dir_all(&directory_path) {
            eprintln!("Error al crear el directorio: {}", e);
        }
    }

    println!("DIR: {:?}", directory_path);
    directory_path.push(format!("{}.svelte", component));

    println!("FILE: {:?}", directory_path);

    if directory_path.exists() {
        return Vec::new();
    }

    match read_to_string(format!("./src/templates/component.svelte")) {
        Ok(content) => {
            // replace content search ".name" with component
            let content = content.replace("name", &component);
            if let Err(e) = fs::write(&directory_path, content) {
                eprintln!("Error al escribir en el archivo: {}", e);
                return Vec::new();
            }
        }
        Err(e) => {
            eprintln!("Error al leer el archivo: {} - {}", e, component);
            return Vec::new();
        }
    }

    fn_get_routes(directory)
}

#[command]
pub fn create_store(state: State<'_, DatabaseConnection>, component: String, directory: String) -> Vec<String> {
    let component = component.to_case(Case::UpperCamel);
    let mut directory_path = PathBuf::from(&directory);
    directory_path.push("src/lib/stores");


    if !directory_path.exists() {
        if let Err(e) = fs::create_dir_all(&directory_path) {
            eprintln!("Error al crear el directorio: {}", e);
        }
    }

    println!("DIR: {:?}", directory_path);
    directory_path.push(format!("{}.svelte.ts", component.to_case(convert_case::Case::Camel)));

    println!("FILE: {:?}", directory_path);

    if directory_path.exists() {
        return Vec::new();
    }

    match read_to_string(format!("./src/templates/store.svelte.ts")) {
        Ok(content) => {
            // replace content search ".name" with component
            let content = content
                .replace("EXPORT_NAME_STORE", &format!("{}Store", component.to_case(Case::Camel)))
                .replace("NAME_STORE", &component);

            if let Err(e) = fs::write(&directory_path, content) {
                eprintln!("Error al escribir en el archivo: {}", e);
                return Vec::new();
            }
        }
        Err(e) => {
            eprintln!("Error al leer el archivo: {} - {}", e, component);
            return Vec::new();
        }
    }

    fn_get_routes(directory)
}

