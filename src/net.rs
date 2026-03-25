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

pub fn download_latest_claude() {
    let url = "https://github.com/cyphertrade/backend-prompt/archive/refs/heads/main.zip";

    let response = reqwest::blocking::get(url).unwrap();

    if !response.status().is_success() {
        panic!(
            "Failed to download zip file from {}. Http Status is: {}",
            url,
            response.status()
        );
    }

    let zip_bytes = response.bytes().unwrap();

    let cursor = Cursor::new(zip_bytes);
    let mut archive = zip::ZipArchive::new(cursor)
        .unwrap_or_else(|err| panic!("Failed to open zip archive. Err: {err}"));

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)
            .unwrap_or_else(|err| panic!("Failed to read file at index {i} from zip. Err: {err}"));

        let file_path = file.name().to_string();

        // Strip the top-level directory (backend-prompt-main/) and extract to root
        let stripped_path = match file_path.find('/') {
            Some(pos) => &file_path[pos + 1..],
            None => continue,
        };

        if stripped_path.is_empty() {
            continue;
        }

        let target_path = Path::new(stripped_path);

        if file.is_dir() {
            fs::create_dir_all(target_path)
                .unwrap_or_else(|err| panic!("Failed to create directory {target_path:?}. Err: {err}"));
            continue;
        }

        if let Some(parent) = target_path.parent() {
            fs::create_dir_all(parent)
                .unwrap_or_else(|err| panic!("Failed to create directory {parent:?}. Err: {err}"));
        }

        let mut file_contents = Vec::new();
        file.read_to_end(&mut file_contents)
            .unwrap_or_else(|err| panic!("Failed to read file contents from zip. Err: {err}"));

        let mut output_file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(target_path)
            .unwrap_or_else(|err| panic!("Failed to create file {target_path:?}. Err: {err}"));

        output_file.write_all(&file_contents)
            .unwrap_or_else(|err| panic!("Failed to write to file {target_path:?}. Err: {err}"));

        output_file.flush()
            .unwrap_or_else(|err| panic!("Failed to flush file {target_path:?}. Err: {err}"));
    }

    println!("Successfully downloaded and extracted claude instructions to root directory");
}
