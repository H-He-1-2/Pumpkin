use std::io::Write;

use crate::ser::NetworkWriteExt;
use crate::{ClientPacket, WritingError};
use pumpkin_data::packet::clientbound::PLAY_SELECT_ADVANCEMENTS_TAB;
use pumpkin_macros::java_packet;
use pumpkin_util::version::MinecraftVersion;

#[java_packet(PLAY_SELECT_ADVANCEMENTS_TAB)]
pub struct CSelectAdvancementsTab<'a> {
    pub tab_id: Option<&'a str>,
}

impl<'a> CSelectAdvancementsTab<'a> {
    #[must_use]
    pub const fn new(tab_id: Option<&'a str>) -> Self {
        Self { tab_id }
    }

    pub fn empty() -> Self {
        Self { tab_id: None }
    }
}

impl ClientPacket for CSelectAdvancementsTab<'_> {
    fn write_packet_data(
        &self,
        write: impl Write,
        _version: &MinecraftVersion,
    ) -> Result<(), WritingError> {
        let mut writer = write;

        match self.tab_id {
            Some(id) => {
                writer.write_bool(true)?;
                writer.write_string(id)?;
            }
            None => writer.write_bool(false)?,
        }

        Ok(())
    }
}
