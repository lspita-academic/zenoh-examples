use std::env;

fn main() {
    // if let Ok(esp32_ld_library_path) = env::var("ESP32_LD_LIBRARY_PATH") {
    //     println!("cargo::rustc-link-search={esp32_ld_library_path}");
    //     println!("cargo::rerun-if-env-changed=ESP32_LD_LIBRARY_PATH");
    //     let original_ld_library_path = env::var("LD_LIBRARY_PATH").ok();
    //     let ld_library_path = env::join_paths(
    //         [Some(esp32_ld_library_path), original_ld_library_path]
    //             .iter()
    //             .flatten(),
    //     )
    //     .unwrap();
    //     unsafe {
    //         env::set_var("LD_LIBRARY_PATH", ld_library_path.as_os_str());
    //     }
    //     println!("cargo::warning=LD: {:?}", ld_library_path);
    // }
    embuild::espidf::sysenv::output();
}
