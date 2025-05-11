#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct Version {
    pub ytdlp: String,
    pub ffmpeg: String,
}

use tokio::process::Command;

pub async fn version() -> Option<Version> {
    let (ytdlp, ffmpeg) = tokio::join!(
        Command::new("yt-dlp").arg("--version").output(),
        Command::new("ffmpeg").arg("-version").output()
    );

    let ytdlp = unsafe { String::from_utf8_unchecked(ytdlp.ok()?.stdout) };
    let ffmpeg = unsafe { String::from_utf8_unchecked(ffmpeg.ok()?.stdout) };
    Some(Version { ytdlp, ffmpeg })
}
