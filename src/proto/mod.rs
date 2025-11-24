mod fetch;

use std::path::Path;
use tonic_prost_build::Builder;

pub fn sync_and_build_proto_file(url_resource: &str, proto_file_name: &str) {
    let proto_path = fetch::download_from_base_url(url_resource, proto_file_name);
    build_proto_from_file(proto_path.as_str());
    println!("Proto file {} is compiled", proto_file_name);
}

pub fn sync_and_build_proto_file_from_private_github_repo(
    repo_owner_name: &str,
    repo_name: &str,
    file_path: &str,
) {
    let proto_path = fetch::download_from_private_github(repo_owner_name, repo_name, file_path);
    build_proto_from_file(proto_path.as_str());
    println!("Proto file {} is compiled", file_path);
}

pub fn build_proto_from_file(path: &str) {
    let proto_path = Path::new(path);
    let proto_dir = proto_path
        .parent()
        .expect("proto file should reside in a directory");

    tonic_prost_build::configure()
        .compile_protos(&[proto_path], &[proto_dir])
        .unwrap();
}

pub fn sync_and_build_proto_file_with_builder(
    url_resource: &str,
    proto_file_name: &str,
    builder: impl Fn(Builder) -> Builder,
) {
    let proto_path = fetch::download_from_base_url(url_resource, proto_file_name);
    compile_with_builder(proto_path.as_ref(), builder);
    println!("Proto file {} is compiled", proto_file_name);
}

fn compile_with_builder(proto_path: &Path, builder: impl Fn(Builder) -> Builder) {
    let proto_dir = proto_path
        .parent()
        .expect("proto file should reside in a directory");

    builder(tonic_prost_build::configure())
        .compile_protos(&[proto_path], &[proto_dir])
        .unwrap();
}
