use std::process::{Command, exit};
use std::fs::{self, File};
use std::io::{Write};
use crate::video_logger::FileDetails;

pub fn progressive_join(files: Vec<String>, output: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut current_joined_file: Option<String> = None;

    for (index, file) in files.iter().enumerate() {
        let output = format!("{}/output_{}.mp4", output,index);

        match index {
            0 => {
                current_joined_file = Some(file.clone());
            }
            1 => {
                // Join the first two files using FFmpeg in terminal
                join_two_files_terminal(&current_joined_file.as_ref().unwrap(), &file, &output)?;
                fs::remove_file(current_joined_file.as_ref().unwrap())?;
                current_joined_file = Some(output);
            }
            _ => {
                // Join the remaining files
                join_two_files_terminal(&current_joined_file.as_ref().unwrap(), &file, &output)?;
                fs::remove_file(current_joined_file.as_ref().unwrap())?;
                current_joined_file = Some(output);
            }
        }
    }

    // Rename the final output file
    fs::rename(current_joined_file.unwrap(), output)?;

    Ok(())
}

fn join_two_files_terminal(input1: &str, input2: &str, output: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Create a temporary file list for FFmpeg
    let mut file_list = File::create("mylist.txt")?;
    
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