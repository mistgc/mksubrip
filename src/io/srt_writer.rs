use crate::prelude::*;
use crate::Writer;

use std::fs;
use std::io::Write;
use std::path::Path;

pub struct SrtWriter {
    file: fs::File,
}

impl Writer for SrtWriter {
    fn write(&mut self, subrip: &crate::Subrip) -> Result<()> {
        if subrip.format != crate::subrip::SubripFormat::SRT {
            return Err(anyhow!("Invalid format (expected SRT)"));
        }
        let index = subrip.index;
        let begin_time = subrip.begin_time.format("%H:%M:%S,%3f").to_string();
        let end_time = subrip.end_time.format("%H:%M:%S,%3f").to_string();
        let content = &subrip.content;

        let str = format!(
            r#"{index}
{begin_time} --> {end_time}
{content}

"#
        );

        write!(self.file, "{}", str)?;

        Ok(())
    }

    fn write_multi(&mut self, subrips: &[crate::Subrip]) -> Result<()> {
        for subrip in subrips.iter() {
            self.write(subrip)?
        }

        Ok(())
    }
}

impl SrtWriter {
    pub fn new(path: &Path) -> Result<Self> {
        let file = fs::File::create(path)?;

        let writer = Self { file };

        Ok(writer)
    }
}
