use anyhow::{Context, Result};
use log::{info, warn};
use rodio::{OutputStream, OutputStreamBuilder};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::mpsc::Sender;

pub struct Speaker {
    tx: Sender<String>,
}

impl Speaker {
    pub fn new() -> Result<Self> {
        let stream_handle =
            OutputStreamBuilder::open_default_stream().context("open default stream failed")?;
        let tx = Self::init_thread(stream_handle);

        Ok(Self { tx })
    }

    fn init_thread(stream_handle: OutputStream) -> Sender<String> {
        let (tx, rx) = std::sync::mpsc::channel::<String>();
        std::thread::spawn(move || -> Result<()> {
            while let Ok(audio_path) = rx.recv() {
                let file = BufReader::new(File::open(&audio_path).context("open file failed")?);
                info!("play audio: {}", &audio_path);
                let sink = rodio::play(stream_handle.mixer(), file).context("play wav failed")?;
                sink.set_volume(1.7);
                sink.set_speed(1.05);
                sink.sleep_until_end();
            }

            Ok(())
        });
        tx
    }

    pub fn play_wav(&self, path: &str) -> Result<()> {
        if Path::new(path).exists() {
            self.tx.send(path.to_string())?;
        } else {
            warn!("audio file not found: {}", path)
        }
        Ok(())
    }
}
