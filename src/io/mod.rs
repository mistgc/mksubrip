mod srt_writer;

use chrono::Timelike;

use crate::io::srt_writer::SrtWriter;
use crate::prelude::*;
use crate::subrip::SubripFormat;

use std::path::{Path, PathBuf};

pub trait Writer {
    fn write(&mut self, subrip: &crate::Subrip) -> Result<()>;

    fn write_multi(&mut self, subrips: &[crate::Subrip]) -> Result<()>;
}

pub struct SubripSaveHelper {
    save_dir_path: PathBuf,
    writers: Vec<Box<dyn Writer>>,
    app_state: Shared<crate::app::AppState>,
}

pub struct SubripWriterBuilder {}

impl SubripSaveHelper {
    pub fn new(app_state: Shared<crate::app::AppState>) -> Self {
        let doc_dir = dirs::download_dir().unwrap_or(PathBuf::from("./"));

        Self {
            save_dir_path: doc_dir,
            writers: vec![],
            app_state,
        }
    }

    pub fn save_dir_path(&self) -> &Path {
        self.save_dir_path.as_path()
    }

    pub fn add_writer(&mut self, writer: Box<dyn Writer>) {
        self.writers.push(writer);
    }

    pub fn save(&mut self, _: &()) {
        let mut subrips = vec![];

        if self.app_state.borrow().subrips.is_empty() {
            error!("Subrips are empty!!!");

            return;
        }

        info!("Save to {}", self.save_dir_path.to_str().unwrap_or("null"));

        self.app_state.borrow().subrips.iter().for_each(|i| {
            let subrip = i.borrow().clone();
            subrips.push(subrip);
        });

        // Re-sort subrips by `begin_time` of the subrip
        subrips.sort_by(|a, b| {
            let t0 = a.begin_time.num_seconds_from_midnight();
            let t1 = b.begin_time.num_seconds_from_midnight();

            t0.cmp(&t1)
        });

        // Rectify the index field of the subrip in subrips.
        for (k, v) in subrips.iter_mut().enumerate() {
            v.set_index((k + 1) as u32);
        }

        for writer in self.writers.iter_mut() {
            if let Err(err) = writer.write_multi(&subrips) {
                error!("{}", err.to_string());
            }
        }
    }
}

impl SubripWriterBuilder {
    pub fn generate_writer_from_format(
        save_dir_path: &std::path::Path,
        format: SubripFormat,
    ) -> Option<Box<dyn Writer>> {
        match format {
            SubripFormat::SRT => {
                let file_path = save_dir_path.join("a.srt");
                if let Ok(srt_writer) = SrtWriter::new(file_path.as_path()) {
                    Some(Box::new(srt_writer))
                } else {
                    None
                }
            }
        }
    }
}
