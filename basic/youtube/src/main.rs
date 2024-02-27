use youtube_dl::YoutubeDl;

fn main() {
    let output = YoutubeDl::new("https://www.youtube.com/watch?v=9hf0b5d2MuA")
        .socket_timeout("15")
        .run()
        .unwrap();
    let sv = output.into_single_video().unwrap();
    if let Some(tg) = sv.categories {
        for tag in tg {
            if let Some(tag) = tag {
                println!("Video title: {:#?}", tag);
            }
        }
    }
}
