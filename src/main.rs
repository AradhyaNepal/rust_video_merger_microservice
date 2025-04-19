mod video_fetcher;
use video_fetcher::fetch_videos;

fn main() {
    println!("Hello, world!");
    let input="/Users/aradhyagopal/Rust/Resources";
      let value = fetch_videos(&input).unwrap();
      for e in value{
        println!("{}", e.display());
      }

}
