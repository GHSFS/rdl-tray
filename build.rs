//! Embeds the Win32 manifest and version info from `resources/app.rc`.

fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        embed_resource::compile("resources/app.rc", embed_resource::NONE);
        println!("cargo:rerun-if-changed=resources/app.rc");
        println!("cargo:rerun-if-changed=resources/app.manifest");
    }
}
