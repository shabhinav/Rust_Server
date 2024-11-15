use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

pub struct StaticFileServer {
    public_dir: String,
}

impl StaticFileServer {
    pub fn new(public_dir: &str) -> Self {
        StaticFileServer {
            public_dir: public_dir.to_string(),
        }
    }

    pub fn serve_file(&self, file_path: &str) -> io::Result<(String, Vec<u8>)> {
        let path = file_path.trim_start_matches('/');
        
        // Handle root path request by serving index.html
        let path = if path.is_empty() {
            "index.html"
        } else {
            path
        };

        let full_path = Path::new(&self.public_dir).join(path);
        println!("URL FILE PATH: {:?}", full_path); // Fixed println! syntax and added better formatting

        // Security check: ensure the path is within public_dir
        let canonical_path = full_path.canonicalize().map_err(|e| {
            println!("Path canonicalization error: {:?}", e);
            io::Error::new(io::ErrorKind::NotFound, "File not found")
        })?;
        
        let public_canonical = Path::new(&self.public_dir).canonicalize().map_err(|_| {
            io::Error::new(io::ErrorKind::Other, "Server configuration error")
        })?;

        if !canonical_path.starts_with(&public_canonical) {
            println!("Access denied: Path traversal attempted");
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "Access denied",
            ));
        }

        // Try to open and read the file
        let mut file = File::open(&full_path).map_err(|e| {
            println!("File open error: {:?}", e);
            e
        })?;
        
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).map_err(|e| {
            println!("File read error: {:?}", e);
            e
        })?;

        // Determine content type
        let content_type = match full_path.extension().and_then(|e| e.to_str()) {
            Some("html") => "text/html",
            Some("css") => "text/css",
            Some("js") => "application/javascript",
            Some("png") => "image/png",
            Some("jpg") | Some("jpeg") => "image/jpeg",
            Some("svg") => "image/svg+xml",     // Added SVG support
            Some("json") => "application/json",  // Added JSON support
            Some("txt") => "text/plain",        // Added text file support
            Some("pdf") => "application/pdf",    // Added PDF support
            _ => "application/octet-stream",
        };

        println!("Successfully serving file: {:?} as {}", full_path, content_type);
        Ok((content_type.to_string(), contents))
    }
}