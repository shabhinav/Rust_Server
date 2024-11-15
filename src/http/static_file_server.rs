use std::fs::File;                    // For reading files
use std::io::{self, Read};           // For input/output operations
use std::path::Path;                  // For handling file paths

pub struct StaticFileServer {
    public_dir: String,               // Where our files are stored
}

impl StaticFileServer {
    pub fn new(public_dir: &str) -> Self {
        StaticFileServer {
            public_dir: public_dir.to_string(),
        }
    }

    pub fn serve_file(&self, file_path: &str) -> io::Result<(String, Vec<u8>)> {
        let path = file_path.trim_start_matches('/');

        let full_path = Path::new(&self.public_dir).join(path);
        println("URL FILE PATH {}:",full_path);
        let canonical_path = full_path.canonicalize()?;
        if !canonical_path.starts_with(Path::new(&self.public_dir)) {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "Access denied",
            ));
        }

        let mut file = File::open(&full_path)?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;

        let content_type = match full_path.extension().and_then(|e| e.to_str()) {
            Some("html") => "text/html",          // HTML files
            Some("css") => "text/css",           // CSS files
            Some("js") => "application/javascript", // JavaScript files
            Some("png") => "image/png",          // PNG images
            Some("jpg") | Some("jpeg") => "image/jpeg", // JPEG images
            _ => "application/octet-stream",     // Unknown file types
        };

        // Return the content type and file contents
        Ok((content_type.to_string(), contents))
    }
    
}