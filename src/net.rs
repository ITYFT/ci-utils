use std::fs::{self, OpenOptions};
use std::io::{Cursor, Read, Write};
use std::path::Path;

pub fn download_file(url_resource: &str, dest_path: &str) {
    let response = reqwest::blocking::get(url_resource).unwrap();

    if !response.status().is_success() {
        panic!(
            "Failed to download file {}. Http Status is: {}",
            url_resource,
            response.status()
        );
    }

    let content = response.text().unwrap();

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(dest_path)
        .unwrap_or_else(|err| panic!("Failed to open file {dest_path}. Err: {err}"));

    file.write_all(content.as_bytes())
        .unwrap_or_else(|err| panic!("Failed to write to file {dest_path}. Err: {err}"));

    file.flush()
        .unwrap_or_else(|err| panic!("Failed to flush to file {dest_path}. Err: {err}"));
}

pub fn download_latest_cursor_instructions() {
    let url = "https://github.com/cyphertrade/cursor-rules/archive/refs/heads/master.zip";
    
    // Download the zip file
    let response = reqwest::blocking::get(url).unwrap();
    
    if !response.status().is_success() {
        panic!(
            "Failed to download zip file from {}. Http Status is: {}",
            url,
            response.status()
        );
    }
    
    let zip_bytes = response.bytes().unwrap();
    
    // Open the zip archive from memory
    let cursor = Cursor::new(zip_bytes);
    let mut archive = zip::ZipArchive::new(cursor)
        .unwrap_or_else(|err| panic!("Failed to open zip archive. Err: {err}"));
    
    // Remove existing .cursor directory if it exists
    let cursor_dir = Path::new(".cursor");
    if cursor_dir.exists() {
        fs::remove_dir_all(cursor_dir)
            .unwrap_or_else(|err| panic!("Failed to remove existing .cursor directory. Err: {err}"));
    }
    
    // Extract files from the zip
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)
            .unwrap_or_else(|err| panic!("Failed to read file at index {i} from zip. Err: {err}"));
        
        let file_path = file.name().to_string();
        
        // Skip the root directory (cursor-rules-master/) and only extract .cursor directory
        if let Some(stripped_path) = file_path.strip_prefix("cursor-rules-master/.cursor/") {
            // Handle directories
            if file.is_dir() {
                let target_path = Path::new(".cursor").join(stripped_path);
                fs::create_dir_all(&target_path)
                    .unwrap_or_else(|err| panic!("Failed to create directory {target_path:?}. Err: {err}"));
                continue;
            }
            
            let target_path = Path::new(".cursor").join(stripped_path);
            
            // Create parent directories if they don't exist
            if let Some(parent) = target_path.parent() {
                fs::create_dir_all(parent)
                    .unwrap_or_else(|err| panic!("Failed to create directory {parent:?}. Err: {err}"));
            }
            
            // Write file contents
            let mut file_contents = Vec::new();
            file.read_to_end(&mut file_contents)
                .unwrap_or_else(|err| panic!("Failed to read file contents from zip. Err: {err}"));
            
            let mut output_file = OpenOptions::new()
                .write(true)
                .truncate(true)
                .create(true)
                .open(&target_path)
                .unwrap_or_else(|err| panic!("Failed to create file {target_path:?}. Err: {err}"));
            
            output_file.write_all(&file_contents)
                .unwrap_or_else(|err| panic!("Failed to write to file {target_path:?}. Err: {err}"));
            
            output_file.flush()
                .unwrap_or_else(|err| panic!("Failed to flush file {target_path:?}. Err: {err}"));
        }
    }
    
    println!("Successfully downloaded and extracted cursor instructions to .cursor directory");
}
