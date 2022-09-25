//! Youtube generator function

use youtube_dl::SearchOptions;
use youtube_dl::{YoutubeDl, YoutubeDlOutput};
use rand::{seq::SliceRandom, thread_rng};

pub fn youtube_video() -> String {
    let mut links: Vec<String> = vec![];

    let charset = vec![
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r",
        "s", "t", "u", "v", "w", "x", "y", "z", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J",
        "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "1", "2",
        "3", "4", "5", "6", "7", "8", "9", "0",
    ];

    let rand_prefix = charset.choose(&mut rand::thread_rng()).unwrap();

    let search_string: String = rand_prefix.to_string();

    let search = SearchOptions::youtube(search_string).with_count(1);
    let output = YoutubeDl::search_for(&search)
        .socket_timeout("15")
        .run()
        .unwrap();

    let entries = match output {
        YoutubeDlOutput::Playlist(videos) => videos.entries,
        _ => panic!("single video should not be a playlist"),
    };

    for video in entries.unwrap() {
        links.push(video.webpage_url.unwrap());
    }

    return String::from(links.choose(&mut thread_rng()).unwrap());
}