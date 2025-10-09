use std::env::var;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::fs;

const REPO_URL: &str = "https://github.com/theparadigmshifters/l0-prover";
const BRANCH: &str = "main";
const REPO_DIR: &str = "l0-prover";

fn main() {
    let out_dir = PathBuf::from(var("OUT_DIR").unwrap());
    let repo_path = out_dir.join(REPO_DIR);

    setup_repository(&repo_path);

    let go_pkg_path = find_main_go(&repo_path)
        .expect("Could not find main.go");

    download_go_dependencies(&go_pkg_path);
    build_go_library(&go_pkg_path, &out_dir);

    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=proof");

    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=framework=Security");
    }

    println!("cargo:rustc-link-lib=dylib=pthread");
    println!("cargo:rustc-link-lib=dylib=dl");

    println!("cargo:rerun-if-changed={}", go_pkg_path.join("main.go").display());
}

fn setup_repository(repo_path: &Path) {
    if !repo_path.exists() {
        eprintln!("Cloning repository...");
        assert!(
            Command::new("git")
                .args([
                    "clone",
                    "--branch", BRANCH,
                    "--depth", "1",
                    REPO_URL,
                    repo_path.to_str().unwrap(),
                ])
                .status()
                .unwrap()
                .success()
        );
    }
}

fn find_main_go(repo_path: &Path) -> Option<PathBuf> {
    let common_paths = [".", "cmd/prover", "prover"];
    
    for path in &common_paths {
        let candidate = repo_path.join(path).join("main.go");
        if candidate.exists() {
            return Some(repo_path.join(path));
        }
    }

    find_recursive(repo_path, 0, 3)
}

fn find_recursive(dir: &Path, depth: usize, max: usize) -> Option<PathBuf> {
    if depth > max || dir.join("main.go").exists() {
        return if dir.join("main.go").exists() { Some(dir.to_path_buf()) } else { None };
    }

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            if entry.file_type().ok()?.is_dir() {
                let name = entry.file_name();
                if name != ".git" && name != "vendor" {
                    if let Some(found) = find_recursive(&entry.path(), depth + 1, max) {
                        return Some(found);
                    }
                }
            }
        }
    }
    None
}

fn download_go_dependencies(go_src: &Path) {
    let go_mod_dir = if go_src.join("go.mod").exists() {
        go_src
    } else if let Some(parent) = go_src.parent() {
        if parent.join("go.mod").exists() { parent } else { go_src }
    } else {
        go_src
    };

    eprintln!("Downloading Go dependencies...");
    
    Command::new("go")
        .args(["mod", "download"])
        .current_dir(go_mod_dir)
        .status()
        .expect("go mod download failed");

    Command::new("go")
        .args(["mod", "tidy"])
        .current_dir(go_mod_dir)
        .status()
        .expect("go mod tidy failed");
}

fn build_go_library(go_src: &Path, out_dir: &Path) {
    let lib_path = out_dir.join("libproof.a");

    eprintln!("Building Go library...");

    assert!(
        Command::new("go")
            .args([
                "build",
                "-buildmode=c-archive",
                "-o",
                lib_path.to_str().unwrap(),
            ])
            .current_dir(go_src)
            .status()
            .unwrap()
            .success(),
        "Go build failed"
    );
}