mod video_fetcher;
mod video_logger;
mod video_merger;
use video_merger::progressive_join;
use video_fetcher::fetch_videos;
use video_logger::{save_file_details, FileDetails,FileStatus};
use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() < 3 {
    eprintln!("Please provide two inputs.");
    std::process::exit(1);
    }
    let input = &args[1];
    let output = &args[2];
    let value:Vec<FileDetails> = fetch_videos(&input)
    .unwrap()
    .iter()
    .enumerate()
    .map(|(index,e)| {
    let metadata = std::fs::metadata(&e);
let file_size = metadata.unwrap().len(); // in bytes
    FileDetails{
      order:index as i32,
      path:e.to_str().unwrap_or_default().to_string(),
      file_size:file_size,
      status:FileStatus::Todo,
      remarks:None,
      started_at:None,
      ended_at:None
    }
  })
    .collect();
  save_file_details(input, &value);

  progressive_join(value.iter().map(|e|e.path.clone()).collect(),output);

}
