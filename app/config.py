# will contain all config options with multiple choices available;
# modifiable by command line arguments

# adding everything availble in the yt_dlp docs though i highly doubt
# anyone would want to download songs at anything below 128kbps
audio_bitrates = ["best", "320", "256", "192", "128", "96", "32", "worst"]

audio_formats = ["mp3", "flac", "m4a"]


# regex patterns for spotify URLs
spotify_link_patterns = [
    # r"^(spotify:|[a-z]+\.spotify\.com/)",
    r"^(spotify:|[a-z]+\.spotify\.com/track/)",
    r"^(spotify:|[a-z]+\.spotify\.com/album/)",
    r"^(spotify:|[a-z]+\.spotify\.com/playlist/)"
    # r"^(spotify:|https://[a-z]+\.spotify\.com/)",
    r"^(spotify:|https://[a-z]+\.spotify\.com/album/)",
    r"^(spotify:|https://[a-z]+\.spotify\.com/playlist/)",
    r"^(spotify:|https://[a-z]+\.spotify\.com/track/)",
]
