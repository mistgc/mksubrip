use std::fs;
use std::io::Write;

use anyhow::{anyhow, Result};

pub trait Writer {
    fn write(&mut self, subrip: &crate::subrip::Subrip) -> Result<()>;

    fn write_multi(&mut self, subrips: &mut Vec<crate::subrip::Subrip>) -> Result<()>;
}

pub struct SrtWriter {
    file: fs::File,
}

impl SrtWriter {
    pub fn new(file: fs::File) -> Result<Self> {
        Ok(Self { file })
    }
}

impl Writer for SrtWriter {
    fn write(&mut self, subrip: &crate::subrip::Subrip) -> Result<()> {
        if subrip.format != crate::subrip::Format::SRT {
            return Err(anyhow!("Invalid format (expected SRT)"));
        }

        let index = subrip.index;
        let (beg_datetime, end_datetime) = subrip.get_time();
        let begin_time = beg_datetime
            .unwrap()
            .time()
            .format("%H:%M:%S,%3f")
            .to_string();
        let end_time = end_datetime
            .unwrap()
            .time()
            .format("%H:%M:%S,%3f")
            .to_string();
        let content = subrip.get_content().unwrap();

        let fmt = format!(
            "\
        {index}\n\
        {begin_time} --> {end_time}\n\
        {content}\n\
        \n\
        "
        );

        write!(self.file, "{}", fmt).unwrap();

        Ok(())
    }

    fn write_multi(&mut self, subrips: &mut Vec<crate::subrip::Subrip>) -> Result<()> {
        for subrip in subrips {
            self.write(subrip)?;
        }

        Ok(())
    }
}
