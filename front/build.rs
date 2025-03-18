use dotenv::dotenv;
use std::env;
use std::fs::File;
use std::io::Write;

fn main() {
    println!("cargo:rerun-if-changed=.env");
    let dest_path = "./src/env.rs";
    let mut f = File::create(dest_path).unwrap();

    // use the dotenv crate to get the .env values
    dotenv().ok();
    f.write_all(b"// This file is automatically generated by build.rs\n\n")
        .unwrap();
    let mut api_url_flag = false;
    for (key, value) in env::vars() {
        if key == "API_URL" {
            let line = format!(
                "pub const {}: &str = \"{}\";\n",
                key,
                value.replace("\"", "\\\"")
            );
            f.write_all(line.as_bytes()).unwrap();
            api_url_flag = true;
            break;
        }
    }
    if !api_url_flag {
        f.write_all(b"pub const API_URL: &str = \"http://localhost:3000/api/\";\n")
            .unwrap();
    }
}
