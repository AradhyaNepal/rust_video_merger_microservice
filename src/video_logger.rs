use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
use chrono::Utc;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug,Clone,PartialEq)]
pub enum FileStatus{
    Todo,
    Pending,
    Error,
    Completed,
}
#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct FileDetails {
    pub order:i32,
    pub path: String,
    pub file_size: u64,
    pub status: FileStatus,
    pub remarks: Option<String>,
    pub started_at: Option<String>,
    pub ended_at: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Meta {
    pub created_at: String,
    pub at_order:i32,
    pub last_updated: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct FileWrapper {
    pub files: HashMap<i32,FileDetails>,
    pub meta: Meta,
}

pub fn read_file_details(location: &str) -> io::Result<Option<FileWrapper>> {
    let location = format!("{}/report.json", location);

    let content = match File::open(&location) {
        Ok(mut file) => {
            let mut buf = String::new();
            file.read_to_string(&mut buf)?;
            buf
        },
        Err(ref e) if e.kind() == io::ErrorKind::NotFound => return Ok(None), // file doesn't exist yet
        Err(e) => return Ok(None),
    };

    if content.trim().is_empty() {
        return Ok(None);
    }

    let wrapper: FileWrapper = match serde_json::from_str(&content){
        Ok(data)=>data,
        Err(e)=>{
            return Ok(None);
        }
    };
    Ok(Some(wrapper))
}

pub fn save_file_details(location: &str,items:&Vec<FileDetails>,at_order:i32) -> io::Result<FileWrapper> {
    let mut existing_data = String::new();
    let location = format!("{}/report.json", location);

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
                at_order:at_order,
                created_at: Utc::now().to_rfc3339(),
                last_updated: None,
            },
        }
    } else {
        serde_json::from_str(&existing_data)?
    };
    for e in items{
        wrapper.files.insert(e.order, e.clone());
    }
    wrapper.meta.at_order=at_order;
    wrapper.meta.last_updated = Some(Utc::now().to_rfc3339());

    // Overwrite the file
    let mut file = File::create(location)?;
    serde_json::to_writer_pretty(&mut file, &wrapper)?;
    Ok(wrapper)

}
