extern crate wayland_scanner;

use std::env::var;
use std::path::Path;

use wayland_scanner::{Side, generate_code};

// Location of the xml file, relative to the `Cargo.toml`
fn main() {
    let protocol_file = "./ext-idle-notify-v1.xml";

    // Target directory for the generate files
    let out_dir_str = var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir_str);

    generate_code(
        protocol_file,
        out_dir.join("idle_client_api.rs"),
        Side::Client, // Replace by `Side::Server` for server-side code
    );
}
