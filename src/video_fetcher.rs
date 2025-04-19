use std::io;
use walkdir::WalkDir;
use std::path:: PathBuf;

pub fn fetch_videos(path:&str)->io::Result<Vec<PathBuf>>{
    let mut mov_files = Vec::new();

    for entry in WalkDir::new(path).into_iter().filter_map(Result::ok) {
        let path = entry.path();

        if path.is_file() && path.extension().map(|ext| ext == "mov").unwrap_or(false) {
            mov_files.push(path.to_path_buf());
        }
    }

    Ok(sort_by_timestamp(mov_files))
}



fn sort_by_timestamp(mut paths: Vec<PathBuf>) -> Vec<PathBuf> {
    paths.sort_by_key(|path| {
        path.file_name()
            .and_then(|name| name.to_str())
            .and_then(|name| name.split('.').next()) // remove ".mov"
            .map(|base| {
                let parts: Vec<&str> = base.split('_').collect();
                if parts.len() == 2 {
                    format!("{}{}", parts[1], parts[0]) // HHMMSS + DDMMYYYY
                } else {
                    String::new()
                }
            })
    });
    paths
}