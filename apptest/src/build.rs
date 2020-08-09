// extern crate dunce;
// use std::{env, path::PathBuf};

fn main() {
    //let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());

    // win32 x64
    // #[cfg(all(target_arch = "x86_64", target_os = "windows"))]
    // let search_path = dunce::canonicalize(root.join("win64")).unwrap();

    // win32 x86
    //#[cfg(all(target_os = "windows", target_arch = "x86"))] // , target_env = "gnu"
    //let search_path = dunce::canonicalize(root.join("win32")).unwrap();

    // linux x64
    //  #[cfg(all(target_arch = "x86_64", target_os = "linux"))]
    //  let search_path = dunce::canonicalize(root.join("linux64")).unwrap();

    //  // linux arm
    //  #[cfg(all(target_arch = "arm", target_os = "linux"))]
    //  let search_path = dunce::canonicalize(root.join("linuxarm")).unwrap();

    //  // macOS x64
    //  #[cfg(all(target_arch = "x86_64", target_os = "macos"))]
    //  let search_path = dunce::canonicalize(root.join("macos64")).unwrap();

    // println!(
    //     "cargo:rustc-link-search=native={}",
    //     env::join_paths(&[search_path]).unwrap().to_str().unwrap()
    // );
}
