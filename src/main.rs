mod video_fetcher;
mod video_logger;
use video_fetcher::fetch_videos;
use video_logger::{save_file_details, FileDetails,FileStatus};

fn main() {
let value=String::from("/Users/aradhyagopal/Rust/Resources");
  save_file_details(&value,FileDetails{
    order:1,
    path:String::from("/Users/aradhyagopal/Rust/Resources/1"),
    status:FileStatus::Pending,
    remarks:String::from("Completed"),
  started_at:String::from("Today"),
    ended_at:String::from("Tommorow")
  });
  // return;
  //   println!("Hello, world!");
  //   let input="/Users/aradhyagopal/Rust/Resources";
  //     let value = fetch_videos(&input).unwrap();
  //     for e in value{
  //       println!("{}", e.display());
  //     }

}
