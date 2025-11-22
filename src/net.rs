use std::fs::OpenOptions;
use std::io::Write;

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
        .unwrap_or_else(|err| panic!("Failed to open file {}. Err: {}", dest_path, err));

    file.write_all(content.as_bytes())
        .unwrap_or_else(|err| panic!("Failed to write to file {}. Err: {}", dest_path, err));

    file.flush()
        .unwrap_or_else(|err| panic!("Failed to flush to file {}. Err: {}", dest_path, err));
}
