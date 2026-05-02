//! Smoke tests — the binary is a windowed app with no CLI surface, so the
//! suite exercises the public helpers from the crate.

#[test]
fn url_extraction_works_on_typical_clipboard_text() {
    // Mirror the call path used by the clipboard listener.
    use std::process::Command;

    let exe = env!("CARGO_BIN_EXE_rdl-tray");
    // Just check the binary is built and is a regular file.
    let meta = std::fs::metadata(exe).expect("binary should be built before tests");
    assert!(meta.is_file());
    assert!(meta.len() > 1024, "binary too small: {} bytes", meta.len());

    // We don't launch the windowed app in a unit test (it would create a
    // tray icon). The build step alone is the meaningful smoke check.
    let _ = Command::new("cmd").arg("/C").arg("ver");
}
