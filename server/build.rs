fn main() {
    println!("cargo:rerun-if-env-changed=STREAM_SERVER_SKIP_WINDOWS_RESOURCES");

    if let Ok(output) = std::process::Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
    {
        if output.status.success() {
            let sha = String::from_utf8_lossy(&output.stdout);
            println!("cargo:rustc-env=GIT_SHA={}", sha.trim());
        }
    }

    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    // The server crate is also embedded as a library by desktop clients. Its
    // standalone EXE resources must not propagate into the host executable,
    // where they would collide with the host's VERSIONINFO/icon/manifest.
    let skip_windows_resources =
        std::env::var_os("STREAM_SERVER_SKIP_WINDOWS_RESOURCES").is_some();
    if target_os == "windows" && !skip_windows_resources {
        use chrono::Datelike;
        let now = chrono::Local::now();
        let copyright = format!("Copyright © {} Perpetus", now.year());
        let exe_name = format!("{}.exe", std::env::var("CARGO_PKG_NAME").unwrap());

        let mut res = winres::WindowsResource::new();
        res.set_manifest(
            r#"
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
<application xmlns="urn:schemas-microsoft-com:asm.v3">
    <windowsSettings>
        <dpiAware xmlns="http://schemas.microsoft.com/SMI/2005/WindowsSettings">true/pm</dpiAware>
        <dpiAwareness xmlns="http://schemas.microsoft.com/SMI/2016/WindowsSettings">PerMonitorV2</dpiAwareness>
    </windowsSettings>
</application>
</assembly>
"#,
        );
        res.set(
            "FileDescription",
            "Stream Server - Torrent Streaming Server",
        );
        res.set("LegalCopyright", &copyright);
        res.set("OriginalFilename", &exe_name);
        res.set_icon_with_id("../icons/app.ico", "MAINICON");
        res.compile().unwrap();
    }
}
