//! Build script for QuickTransform
//! Embeds Windows icon and sets metadata

fn main() {
    // Windows-specific: embed icon
    #[cfg(target_os = "windows")]
    {
        if std::path::Path::new("assets/lazyfrog-kindware.ico").exists() {
            let mut res = winresource::WindowsResource::new();
            res.set_icon("assets/lazyfrog-kindware.ico");
            res.set("ProductName", "QuickTransform");
            res.set("FileDescription", "Lightning-fast encoder/decoder/hasher");
            res.set("LegalCopyright", "LAZYFROG-kindware.dev - MIT License");
            res.set("CompanyName", "LAZYFROG-kindware.dev");
            if let Err(e) = res.compile() {
                eprintln!("Warning: Failed to embed icon: {}", e);
            }
        }
    }
}
