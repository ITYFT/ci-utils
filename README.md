# ci-utils

A Rust utility library for CI/CD tasks including file downloads, protocol buffer compilation, and JavaScript file merging.

## Public Functions

### Network Utilities (`net` module)

#### `download_file`
Downloads a file from a URL and saves it to a local path.

```rust
pub fn download_file(url_resource: &str, dest_path: &str)
```

**Parameters:**
- `url_resource`: The URL of the file to download
- `dest_path`: The local file path where the downloaded file will be saved

**Behavior:**
- Downloads the file using blocking HTTP request
- Panics if the HTTP request fails or returns a non-success status code
- Overwrites the destination file if it already exists

**Example:**
```rust
use ci_utils::download_file;

download_file("https://example.com/file.txt", "local_file.txt");
```

---

### Protocol Buffer Utilities (`proto` module)

#### `build_proto_from_file`
Compiles a protocol buffer file using `tonic-build`.

```rust
pub fn build_proto_from_file(path: &str)
```

**Parameters:**
- `path`: The path to the `.proto` file to compile

**Behavior:**
- Compiles the proto file using default `tonic-build` configuration
- Panics if compilation fails

**Example:**
```rust
use ci_utils::build_proto_from_file;

build_proto_from_file("proto/my_service.proto");
```

---

#### `sync_and_build_proto_file`
Downloads a protocol buffer file from a base URL and compiles it.

```rust
pub fn sync_and_build_proto_file(url_resource: &str, proto_file_name: &str)
```

**Parameters:**
- `url_resource`: The base URL where the proto file is hosted
- `proto_file_name`: The name of the proto file to download

**Behavior:**
- Downloads the proto file from `{url_resource}{proto_file_name}`
- Saves the file to the `proto/` directory
- Compiles the downloaded proto file
- Prints a success message when compilation completes
- Optionally uses `GIT_HUB_TOKEN` environment variable for authentication if set

**Example:**
```rust
use ci_utils::sync_and_build_proto_file;

sync_and_build_proto_file("https://raw.githubusercontent.com/example/repo/main/", "service.proto");
```

---

#### `sync_and_build_proto_file_from_private_github_repo`
Downloads a protocol buffer file from a private GitHub repository and compiles it.

```rust
pub fn sync_and_build_proto_file_from_private_github_repo(
    repo_owner_name: &str,
    repo_name: &str,
    file_path: &str,
)
```

**Parameters:**
- `repo_owner_name`: The GitHub repository owner or organization name
- `repo_name`: The name of the GitHub repository
- `file_path`: The path to the proto file within the repository (e.g., `"proto/service.proto"`)

**Behavior:**
- Downloads the proto file from the private GitHub repository using the GitHub API
- Requires `GIT_HUB_TOKEN` environment variable to be set (panics if not set)
- Decodes the base64-encoded content from GitHub API response
- Saves the file to the local `proto/` directory
- Compiles the downloaded proto file
- Prints a success message when compilation completes

**Example:**
```rust
use ci_utils::sync_and_build_proto_file_from_private_github_repo;

// Requires GIT_HUB_TOKEN environment variable
sync_and_build_proto_file_from_private_github_repo(
    "myorg",
    "myrepo",
    "proto/service.proto"
);
```

---

#### `sync_and_build_proto_file_with_builder`
Downloads a protocol buffer file from a base URL and compiles it with custom `tonic-build` configuration.

```rust
pub fn sync_and_build_proto_file_with_builder(
    url_resource: &str,
    proto_file_name: &str,
    builder: impl Fn(Builder) -> Builder,
)
```

**Parameters:**
- `url_resource`: The base URL where the proto file is hosted
- `proto_file_name`: The name of the proto file to download
- `builder`: A closure that configures the `tonic_build::Builder` with custom settings

**Behavior:**
- Downloads the proto file from `{url_resource}{proto_file_name}`
- Saves the file to the `proto/` directory
- Compiles the downloaded proto file using the custom builder configuration
- Prints a success message when compilation completes
- Optionally uses `GIT_HUB_TOKEN` environment variable for authentication if set

**Example:**
```rust
use ci_utils::sync_and_build_proto_file_with_builder;

sync_and_build_proto_file_with_builder(
    "https://raw.githubusercontent.com/example/repo/main/",
    "service.proto",
    |builder| builder.build_server(true).build_client(false)
);
```

---

### JavaScript Utilities (`js` module)

#### `merge_js_files`
Merges multiple JavaScript files into a single output file, removing comment lines.

```rust
pub fn merge_js_files(js_files: &[&str], out_file_name: &str)
```

**Parameters:**
- `js_files`: A slice of JavaScript file names (must end with `.js`)
- `out_file_name`: The name of the output file to create

**Behavior:**
- Reads JavaScript files from the `JavaScript/` directory
- Filters out lines that start with `//` (comment lines)
- Writes each file's content to the output file with a header comment indicating the source file
- Panics if any file operations fail

**Example:**
```rust
use ci_utils::js::merge_js_files;

merge_js_files(&["file1.js", "file2.js", "file3.js"], "merged.js");
```

---

## Dependencies

- `tonic-build`: Protocol buffer compilation
- `reqwest`: HTTP client for file downloads
- `serde_json`: JSON parsing for GitHub API responses
- `base64`: Base64 decoding for GitHub content

## Environment Variables

- `GIT_HUB_TOKEN`: Optional GitHub token for authenticated requests. Required for `sync_and_build_proto_file_from_private_github_repo`.

## License

[Add your license information here]

