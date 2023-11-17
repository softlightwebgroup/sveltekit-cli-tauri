use std::fs;

pub fn recursive_routes(path: String, routes: &mut Vec<String>) {
    let paths = fs::read_dir(path).unwrap();

    for path in paths {
        let path = path.unwrap().path();
        let option = path.file_name().unwrap().to_str().unwrap();

        if path.is_dir() {
            routes.push(format!("/{option}"));
            recursive_routes(path.display().to_string(), routes);
        }
    }
}


pub fn fn_get_routes(path: String) -> Vec<String> {
    let pathRoutes = format!("{}/src/routes", path);

    let paths = fs::read_dir(pathRoutes).unwrap();

    let mut routes = Vec::new();

    routes.push(String::from("/"));

    for path in paths {
        let path = path.unwrap().path();
        let option = path.file_name().unwrap().to_str().unwrap();

        if path.is_dir() {
            routes.push(format!("/{option}"));

            recursive_routes(path.display().to_string(), &mut routes);
        }
    }

    routes
}