use std::process::{Command, exit};
use std::fs::{self, File};
use std::io::{Write};
use chrono::Utc;
use crate::video_logger::{save_file_details, FileDetails, FileStatus,FileWrapper};

pub fn progressive_join(files: FileWrapper, output: &str,log_path:&str) -> Result<(), Box<dyn std::error::Error>> {

    let mut current_joined_file: Option<String> = None;
   let last_index=(files.files.len()-1) as i32;
    let is_completed=files.files[&last_index].status==FileStatus::Completed;
    if(is_completed){
        return Ok(());
    }
    for index in 0..=last_index {
        let parsed_index=index as i32;
        if parsed_index<files.meta.at_order{
            continue;
        }
        let file=files.files[&index].clone();
        if file.status == FileStatus::Completed{
            continue;
        }
        let output = format!("{}/output_{}.mp4", output,index);
        match index {
            0 => {
                current_joined_file = Some(file.path.clone());
            }
            1 => {
                let first_file= files.files[&0].clone();
                let now= Some(Utc::now().to_rfc3339());
                let first_file= FileDetails{
                    status:FileStatus::Pending,
                    started_at:now.clone(),
                   ..first_file.clone()
                };

                let file=FileDetails{
                    status:FileStatus::Pending,
                    started_at:now,
                    ..file.clone()
                };
               
                save_file_details(log_path, &vec![
                    first_file.clone(),
                    file.clone()
                ],0);
                // Join the first two files using FFmpeg in terminal
                match join_two_files_terminal(&current_joined_file.as_ref().unwrap(), &file.path, &output){
                    Ok(_)=>{},
                    Err(e)=>{
                        let error=Some(e.to_string());
                        save_file_details(log_path, &vec![
                            FileDetails{
                                status:FileStatus::Error,
                                remarks:error.clone(),
                               ..first_file.clone()
                            },
                            FileDetails{
                                status:FileStatus::Error,
                                remarks:error,
                                ..file.clone()
                            },
                        ],0);
                      return Ok(());
                    }

                }
                current_joined_file = Some(output);
                let now= Some(Utc::now().to_rfc3339());
                save_file_details(log_path, &vec![
                    FileDetails{
                        status:FileStatus::Completed,
                    ended_at:now.clone(),
                       ..first_file
                    },
                    FileDetails{
                        status:FileStatus::Completed,
                        ended_at:now,
                        ..file.clone()
                    },
                ],1);
               
              
            }
            _ => {
                let file=FileDetails{
                    status:FileStatus::Pending,
                    started_at:Some(Utc::now().to_rfc3339()),
                    ..file.clone()
                };
                save_file_details(log_path, &vec![
                    file.clone()
                ],index as i32);
                // Join the remaining files
                match join_two_files_terminal(&current_joined_file.as_ref().unwrap(), &file.path, &output){
                    Ok(_)=>{},
                    Err(e)=>{
                        let error=Some(e.to_string());
                        save_file_details(log_path, &vec![
                            FileDetails{
                                status:FileStatus::Error,
                                remarks:error,
                                ..file.clone()
                            },
                        ],index as i32);
                        return Ok(());
                    }
                };
                fs::remove_file(current_joined_file.as_ref().unwrap())?;
                current_joined_file = Some(output);
                save_file_details(log_path, &vec![
                    FileDetails{
                        status:FileStatus::Completed,
                        ended_at:Some(Utc::now().to_rfc3339()),
                        ..file.clone()
                    }
                ],index as i32);

            }
        }
    }
    let output = format!("{}/output_final.mp4", output);
    fs::rename(current_joined_file.unwrap(), output)?;
    Ok(())
}

fn join_two_files_terminal(input1: &str, input2: &str, output: &str) -> Result<(), Box<dyn std::error::Error>> {
 
 if output.contains("19042025_73756"){
    return  Err("Mocked error".into());
 }
    let mut file_list = File::create("mylist.txt")?;
   // This will panic because of division by zero
    // Add input video paths to the file list
    writeln!(file_list, "file '{}'", input1)?;
    writeln!(file_list, "file '{}'", input2)?;

    // Run the FFmpeg command to join the two videos
    let status = Command::new("ffmpeg")
        .arg("-f")
        .arg("concat")
        .arg("-safe")
        .arg("0")
        .arg("-i")
        .arg("mylist.txt")
        .arg("-c")
        .arg("copy")
        .arg(output)
        .status()?;

    if !status.success() {
        eprintln!("FFmpeg command failed!");
        exit(1);
    }

    // Clean up the temporary file list
    fs::remove_file("mylist.txt")?;

    Ok(())
}