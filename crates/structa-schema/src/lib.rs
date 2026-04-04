pub mod dto;
pub mod route;
pub mod module;
pub mod entity;
pub mod service;
pub mod app;
pub mod generator;

use std::path::{Path, PathBuf};
use std::fs;

#[derive(Debug, Clone)]
pub enum SchemaFile {
    Dto(dto::DtoSchema),
    Route(route::RouteSchema),
    Module(module::ModuleSchema),
    Entity(entity::EntitySchema),
    Service(service::ServiceSchema),
    App(app::AppSchema),
}

pub fn parse_file(path: &Path) -> Result<SchemaFile, String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read file: {}", e))?;
    
    let file_name = path.file_stem()
        .ok_or("Invalid file name")?
        .to_str()
        .unwrap_or("");

    let extension = path.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");

    match extension {
        "structa" => {
            if file_name.ends_with(".dto") {
                let name = file_name.trim_end_matches(".dto");
                dto::parse_dto(name, &content)
                    .map(SchemaFile::Dto)
            } else if file_name.ends_with(".route") {
                let name = file_name.trim_end_matches(".route");
                route::parse_route(name, &content)
                    .map(SchemaFile::Route)
            } else if file_name.ends_with(".module") {
                let name = file_name.trim_end_matches(".module");
                module::parse_module(name, &content)
                    .map(SchemaFile::Module)
            } else if file_name.ends_with(".entity") {
                let name = file_name.trim_end_matches(".entity");
                entity::parse_entity(name, &content)
                    .map(SchemaFile::Entity)
            } else if file_name.ends_with(".service") {
                let name = file_name.trim_end_matches(".service");
                service::parse_service(name, &content)
                    .map(SchemaFile::Service)
            } else if file_name == "app" {
                app::parse_app(&content)
                    .map(SchemaFile::App)
            } else {
                Err(format!("Unknown structa file type: {}", file_name))
            }
        }
        _ => Err(format!("Unknown file extension: {}", extension)),
    }
}

pub fn discover_files(dir: &Path) -> Result<Vec<PathBuf>, String> {
    let mut files = Vec::new();
    
    if !dir.exists() {
        return Ok(files);
    }

    fn walk_dir(dir: &Path, files: &mut Vec<PathBuf>) -> Result<(), String> {
        for entry in fs::read_dir(dir)
            .map_err(|e| format!("Failed to read directory: {}", e))?
        {
            let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
            let path = entry.path();
            
            if path.is_dir() {
                walk_dir(&path, files)?;
            } else if let Some(ext) = path.extension() {
                if ext == "structa" {
                    files.push(path);
                }
            }
        }
        Ok(())
    }

    walk_dir(dir, &mut files)?;
    Ok(files)
}
