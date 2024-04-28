mod srt_writer;

use crate::prelude::*;

pub trait Writer {
    fn write(&mut self, subrip: &crate::Subrip) -> Result<()>;

    fn write_multi(&mut self, subrips: &[crate::Subrip]) -> Result<()>;
}
