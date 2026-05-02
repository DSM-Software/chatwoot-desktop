fn main() {
    // Re-run when any icon file changes so the embedded app icon stays in sync.
    println!("cargo:rerun-if-changed=icons/");
    tauri_build::build()
}
