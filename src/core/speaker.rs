use anyhow::{Context, Result};
use rodio::{Decoder, OutputStream, OutputStreamBuilder};
use std::fs::File;
use std::io::BufReader;

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
        let file = BufReader::new(File::open(path).context("open file failed")?);
        let sink = rodio::play(&self.stream_handle.mixer(), file).context("play wav failed")?;
        sink.set_volume(1.7);
        sink.set_speed(1.05);
        sink.sleep_until_end();
        Ok(())
    }
}
