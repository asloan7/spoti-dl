mod metadata;
mod spotify;
mod utils;

use utils::path_exists;

use std::collections::HashSet;

use pyo3::prelude::*;
use youtube_dl::YoutubeDl;

/// A Python module implemented in Rust.
#[pymodule]
fn spotidl_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(handle_song_download, m)?)?;
    Ok(())
}

#[derive(Debug)]
struct CliArgs {
    download_dir: String,
    codec: String,
    bitrate: String,
}

/// Handles end-to-end flow for a song download: fetches song information from Spotify, prepares the song's source,
/// downloads it and writes song metadata.
#[pyfunction]
fn handle_song_download(
    py: Python<'_>,
    token: String,
    link: String,
    download_dir: String,
    codec: String,
    bitrate: String,
) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        let Some(spotify_id) = utils::parse_link(&link) else {
            println!("Invalid Spotify link type entered!"); 
            return Ok(())
        };

        let illegal_path_chars: HashSet<char> = ['\\', '?', '%', '*', ':', '|', '"', '<', '>', '.']
            .iter()
            .cloned()
            .collect();

        let download_dir = utils::correct_directory_name(&illegal_path_chars, download_dir);
        let args = CliArgs {
            download_dir,
            codec,
            bitrate,
        };

        if let Err(err) = utils::make_download_directories(&args.download_dir) {
            println!("error creating download directories: {err}");
            return Ok(());
        };

        // todo: correct song name too
        let song = spotify::get_song_details(token, spotify_id).await;
        let file_path = format!("{}/{}.{}", &args.download_dir, &song.name, &args.codec);

        if download_song(&file_path, &song, &args).await {
            metadata::add_metadata(&args.download_dir, &file_path, &song).await
        }

        Ok(())
    })
}

async fn download_song(file_path: &str, song: &spotify::SpotifySong, args: &CliArgs) -> bool {
    let file_output_format = format!("{}/{}.%(ext)s", &args.download_dir, &song.name);

    if path_exists(file_path) {
        println!("{} already exists, skipping download", &song.name);
        return false;
    }

    let query = utils::generate_youtube_query(&song.name, &song.artists);
    let search_options = youtube_dl::SearchOptions::youtube(query);

    let mut yt_client = YoutubeDl::search_for(&search_options);
    println!("Starting {} song download", &song.name);

    if let Err(err) = yt_client
        .extract_audio(true)
        .output_template(file_output_format)
        .extra_arg("--audio-format")
        .extra_arg(&args.codec)
        .extra_arg("--audio-quality")
        .extra_arg(&args.bitrate)
        .download_to_async("./")
        .await
    {
        println!("Error downloading {} song: {err}", &song.name);
        return false;
    }
    true
}
