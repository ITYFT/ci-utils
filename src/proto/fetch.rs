use base64::Engine;
use reqwest::blocking::{Client, Response};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

const PROTO_DIR: &str = "proto";

pub(super) fn download_from_base_url(base_url: &str, proto_file_name: &str) -> String {
    let url = format!("{base_url}{proto_file_name}");
    let response = fetch_with_optional_token(&url);
    ensure_success(&response, &url);

    let content = response.text().unwrap();
    println!("Proto file {proto_file_name} is downloaded");

    write_proto_file(proto_file_name, &content)
}

pub(super) fn download_from_private_github(
    repo_owner: &str,
    repo_name: &str,
    file_path: &str,
) -> String {
    let url = format!("https://api.github.com/repos/{repo_owner}/{repo_name}/contents/{file_path}");
    let response = fetch_private_github(&url);
    ensure_success(&response, &url);

    let raw_body = response.text().unwrap();
    let decoded = decode_github_content(raw_body.as_str());

    let proto_file_name = file_path
        .rsplit('/')
        .next()
        .expect("github file path should have a final segment");

    write_proto_file(proto_file_name, &decoded)
}

fn fetch_with_optional_token(url: &str) -> Response {
    if let Ok(git_hub_token) = std::env::var("GIT_HUB_TOKEN") {
        Client::new()
            .get(url)
            .header("Authorization", format!("token {git_hub_token}"))
            .send()
            .unwrap()
    } else {
        reqwest::blocking::get(url).unwrap()
    }
}

fn fetch_private_github(url: &str) -> Response {
    let git_hub_token =
        std::env::var("GIT_HUB_TOKEN").expect("Please set GIT_HUB_TOKEN environment variable");

    Client::new()
        .get(url)
        .header("Accept", "application/vnd.github+json")
        .header("User-Agent", "RustCiBuilder")
        .header("Authorization", format!("Bearer {git_hub_token}"))
        .header("X-GitHub-Api-Version", "2022-11-28")
        .send()
        .unwrap()
}

fn decode_github_content(content: &str) -> String {
    let json_value = serde_json::from_str::<serde_json::Value>(content).unwrap();
    let base64_body = json_value
        .get("content")
        .and_then(|value| value.as_str())
        .expect("github response should include file content");

    decode_base64_lines(base64_body)
}

fn decode_base64_lines(encoded: &str) -> String {
    let mut decoded = String::new();

    for line in encoded.split('\n') {
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(line)
            .unwrap();
        let line_str = String::from_utf8(bytes).unwrap();
        decoded.push_str(&line_str);
    }

    decoded
}

fn write_proto_file(file_name: &str, content: &str) -> String {
    let proto_path = Path::new(PROTO_DIR).join(file_name);

    if let Some(parent) = proto_path.parent() {
        fs::create_dir_all(parent).unwrap_or_else(|err| {
            panic!(
                "Failed to create proto directory {parent:?}. Err: {err}"
            )
        });
    }

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&proto_path)
        .unwrap_or_else(|err| panic!("Failed to open file {proto_path:?}. Err: {err}"));

    file.write_all(content.as_bytes())
        .unwrap_or_else(|err| panic!("Failed to write to file {proto_path:?}. Err: {err}"));
    file.flush()
        .unwrap_or_else(|err| panic!("Failed to flush to file {proto_path:?}. Err: {err}"));

    println!("Proto file {file_name} is updated");

    proto_path_to_string(proto_path)
}

fn proto_path_to_string(path: PathBuf) -> String {
    path.to_string_lossy().into_owned()
}

fn ensure_success(response: &Response, url: &str) {
    if !response.status().is_success() {
        panic!(
            "Failed to download proto file from {}. Http Status is: {}",
            url,
            response.status()
        );
    }
}
