use anyhow::{Context, Result};
use log::{info, warn};
use rodio::{OutputStream, OutputStreamBuilder};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub struct Speaker {
    stream_handle: OutputStream,
}

impl Speaker {
    pub fn new() -> Result<Self> {
        let stream_handle =
            OutputStreamBuilder::open_default_stream().context("open default stream failed")?;

        Ok(Self { stream_handle })
    }

    pub fn play_wav(&self, path: &str) -> Result<()> {
        if Path::new(path).exists() {
            let file = BufReader::new(File::open(path).context("open file failed")?);
            info!("play audio: {}", path);
            let sink = rodio::play(&self.stream_handle.mixer(), file).context("play wav failed")?;
            sink.set_volume(1.7);
            sink.set_speed(1.05);
            sink.sleep_until_end();
        } else {
            warn!("audio file not found: {}", path)
        }
        Ok(())
    }
}
