extern crate zip;

use std::env;
use std::path::PathBuf;

use zip::write;

fn main() {
    let target = env::var("TARGET").unwrap();
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("Failed to find target dir"));

    // Horrible brittle hack to allow me to send the game to friends
    let mut build_output_dir = out_dir.clone();
    build_output_dir.pop();
    build_output_dir.pop();
    build_output_dir.pop();

    let mut src_resource_dir = manifest_dir.clone();
    src_resource_dir.push("resources");
    let mut dst_target_resource_dir = build_output_dir.clone();
    dst_target_resource_dir.push("resources");
    std::fs::create_dir(dst_target_resource_dir.clone());
    copy(&src_resource_dir, &dst_target_resource_dir, ".png");

    if target.contains("pc-windows") {
        let mut lib_dir = manifest_dir.clone();
        let mut dll_dir = manifest_dir.clone();
        if target.contains("msvc") {
            lib_dir.push("msvc");
            dll_dir.push("msvc");
        } else {
            lib_dir.push("gnu-mingw");
            dll_dir.push("gnu-mingw");
        }
        lib_dir.push("lib");
        dll_dir.push("dll");
        if target.contains("x86_64") {
            lib_dir.push("64");
            dll_dir.push("64");
        } else {
            lib_dir.push("32");
            dll_dir.push("32");
        }
        println!("cargo:rustc-link-search=all={}", lib_dir.display());
        copy(&dll_dir, &manifest_dir, ".dll");
        copy(&dll_dir, &build_output_dir, ".dll");
    }
}

fn copy(src_dir: &PathBuf, dst_dir: &PathBuf, file_type: &str) {
    println!("{:?}", src_dir);
    println!("{:?}", dst_dir);
    for entry in std::fs::read_dir(src_dir).expect(&format!("Can't read dir {:?}", src_dir)) {
        let entry_path = entry.expect("Invalid fs entry").path();
        let file_name_result = entry_path.file_name();
        let mut new_file_path = dst_dir.clone();
        if let Some(file_name) = file_name_result {
            let file_name = file_name.to_str().unwrap();
            if file_name.ends_with(file_type) {
                new_file_path.push(file_name);
                std::fs::copy(&entry_path, new_file_path.as_path()).expect(&format!(
                    "Can't copy from dir {:?} to {:?}",
                    src_dir, dst_dir
                ));
            }
        }
    }
}