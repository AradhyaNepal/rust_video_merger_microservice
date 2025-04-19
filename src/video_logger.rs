use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, Read,Write};
use serde_json::json;
use chrono::Utc;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;


#[derive(Serialize, Deserialize, Debug,Clone)]
pub enum FileStatus{
    Todo,
    Pending,
    Error,
    Completed,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct FileDetails {
    pub order:i32,
    pub path: String,
    pub status: FileStatus,
    pub remarks: String,
    pub started_at: String,
    pub ended_at: String,
}
impl FileDetails {
    pub fn copy_with(
        &self,
        order: Option<i32>,
        path: Option<String>,
        status: Option<FileStatus>,
        remarks: Option<String>,
        started_at: Option<String>,
        ended_at: Option<String>,
    ) -> FileDetails {
        FileDetails {
            order: order.unwrap_or(self.order),
            path: path.unwrap_or_else(|| self.path.clone()),
            status: status.unwrap_or_else(|| self.status.clone()),
            remarks: remarks.unwrap_or_else(|| self.remarks.clone()),
            started_at: started_at.unwrap_or_else(|| self.started_at.clone()),
            ended_at: ended_at.unwrap_or_else(|| self.ended_at.clone()),
        }
    }
}
#[derive(Serialize, Deserialize)]
struct Meta {
    created_at: String,
    last_updated: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct FileWrapper {
    files: HashMap<i32, FileDetails>,
    meta: Meta,
}



pub fn save_file_details(location: &String,file_details:FileDetails) -> io::Result<()> {
    let mut existing_data = String::new();
    let location = format!("{}/progress.json", location);

    // Try to open and read existing file
   match File::open(&location) {
        Ok(mut f) => {
            f.read_to_string(&mut existing_data)?;
            f
        },
        Err(_) => {
            File::create(&location)?
        }
    };

    let mut wrapper: FileWrapper = if existing_data.trim().is_empty() {
        FileWrapper {
            files: HashMap::new(),
            meta: Meta {
                created_at: Utc::now().to_rfc3339(),
                last_updated: None,
            },
        }
    } else {
        serde_json::from_str(&existing_data)?
    };
    wrapper.files.insert(file_details.order.clone(), file_details);
    wrapper.meta.last_updated = Some(Utc::now().to_rfc3339());

    // Overwrite the file
    let mut file = File::create(location)?;
    serde_json::to_writer_pretty(&mut file, &wrapper)?;
    Ok(())

}
