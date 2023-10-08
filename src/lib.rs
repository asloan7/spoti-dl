mod downloader;
mod metadata;
mod spotify;
mod types;
mod utils;

use pyo3::prelude::*;
use spotify::LinkType;

/// A Python module implemented in Rust.
#[pymodule]
fn spotidl_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(process_downloads, m)?)?;
    Ok(())
}

/// Processes end-to-end flow for processing song, album and playlist downloads by validating and parsing user input
/// and calling apt functions to process the steps relating to the downloads
#[pyfunction]
fn process_downloads(
    py: Python<'_>,
    token: String,
    link: String,
    download_dir_str: String,
    codec_str: String,
    bitrate_str: String,
    parallel_downloads_str: String,
) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        pyo3_log::init();

        let Some((link_type,spotify_id)) = utils::parse_link(&link) else {
            return Ok(())
        };
        let Some(cli_args) = utils::parse_cli_arguments(download_dir_str, codec_str, bitrate_str, parallel_downloads_str) else {
            return Ok(())
        };

        let mut spotify_client = spotify::generate_client(token);
        spotify_client.config.token_refreshing = false;

        match link_type {
            LinkType::Track => {
                let song = spotify::get_song_details(spotify_id, spotify_client).await;
                downloader::process_song_download(song, cli_args).await;
            }
            LinkType::Album => {
                let album = spotify::get_album_details(spotify_id, spotify_client).await;
                downloader::process_album_download(album, cli_args).await;
            }
            LinkType::Playlist => {
                let playlist = spotify::get_playlist_details(&spotify_client, &spotify_id).await;
                let _ = downloader::process_playlist_download(
                    spotify_id,
                    spotify_client,
                    playlist,
                    cli_args,
                )
                .await;
            }
        }

        Ok(())
    })
}
