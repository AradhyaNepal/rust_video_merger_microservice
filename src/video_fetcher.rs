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

    Ok(mov_files)
}