use std::path::Path;

use prost_build;

fn main() {
    let project_dir = env!("CARGO_MANIFEST_DIR");
    let pb_path = Path::new(project_dir).join("src/pb");
    if !pb_path.exists() {
        std::fs::create_dir_all(pb_path).unwrap();
    }
    prost_build::Config::new()
        .out_dir("src/pb")
        .compile_protos(&["abi.proto"], &["."])
        .unwrap();
}
