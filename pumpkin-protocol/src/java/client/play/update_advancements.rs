use std::io::Write;

use crate::ser::NetworkWriteExt;
use crate::{ClientPacket, WritingError};
use pumpkin_data::packet::clientbound::PLAY_UPDATE_ADVANCEMENTS;
use pumpkin_macros::java_packet;
use pumpkin_util::version::MinecraftVersion;

const SCriteriaIdentifier: &str = "minecraft:criteria_identifier";

fn criterion_to_nbt(identifier: &str) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.push(8);
    write_mc_string(&mut buf, SCriteriaIdentifier);
    write_mc_string(&mut buf, identifier);
    buf
}

fn write_mc_string(buf: &mut Vec<u8>, s: &str) {
    let len = s.len() as u16;
    buf.extend_from_slice(&len.to_be_bytes());
    buf.extend_from_slice(s.as_bytes());
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdvancementFrameType {
    Task,
    Goal,
    Challenge,
}

#[derive(Clone, Debug)]
pub struct Advancement {
    pub id: String,
    pub parent_id: Option<String>,
    pub display_data: Option<AdvancementDisplayData>,
    pub criteria: Vec<String>,
    pub requirements: Vec<Vec<String>>,
}

#[derive(Clone, Debug)]
pub struct AdvancementDisplayData {
    pub title: String,
    pub description: String,
    pub frame_type: AdvancementFrameType,
    pub flags: u8,
    pub icon: String,
    pub background_texture: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Criteria {
    pub identifier: String,
}

impl Criteria {
    pub fn new(identifier: impl Into<String>) -> Self {
        Self {
            identifier: identifier.into(),
        }
    }
}

#[java_packet(PLAY_UPDATE_ADVANCEMENTS)]
pub struct CUpdateAdvancements<'a> {
    pub reset: bool,
    pub advancements: &'a [Advancement],
    pub identifiers: &'a [String],
    pub progress: &'a [(String, Criteria)],
}

impl<'a> CUpdateAdvancements<'a> {
    pub fn new(
        reset: bool,
        advancements: &'a [Advancement],
        identifiers: &'a [String],
        progress: &'a [(String, Criteria)],
    ) -> Self {
        Self {
            reset,
            advancements,
            identifiers,
            progress,
        }
    }
}

impl ClientPacket for CUpdateAdvancements<'_> {
    fn write_packet_data(
        &self,
        mut write: impl Write,
        _version: &MinecraftVersion,
    ) -> Result<(), WritingError> {
        write.write_bool(self.reset)?;
        write.write_var_int(&(self.advancements.len() as i32).into())?;

        for adv in self.advancements {
            write.write_string(&adv.id)?;

            if let Some(ref parent) = adv.parent_id {
                write.write_bool(true)?;
                write.write_string(parent)?;
            } else {
                write.write_bool(false)?;
            }

            if let Some(ref display) = adv.display_data {
                write.write_bool(true)?;
                write.write_string(&display.title)?;
                write.write_string(&display.description)?;

                let frame_int = match display.frame_type {
                    AdvancementFrameType::Task => 0i32,
                    AdvancementFrameType::Goal => 1i32,
                    AdvancementFrameType::Challenge => 2i32,
                };
                write.write_var_int(&frame_int.into())?;

                write.write_u8(display.flags)?;
                write.write_string(&display.icon)?;
                write.write_string(
                    display
                        .background_texture
                        .as_deref()
                        .unwrap_or("minecraft:textures/gui/advancements/task_background.png"),
                )?;
            } else {
                write.write_bool(false)?;
            }

            write.write_var_int(&(adv.criteria.len() as i32).into())?;

            for criterion in &adv.criteria {
                write.write_string(criterion)?;
            }

            write.write_var_int(&(adv.requirements.len() as i32).into())?;

            for req in &adv.requirements {
                write.write_var_int(&(req.len() as i32).into())?;
                for item in req {
                    write.write_string(item)?;
                }
            }
        }

        write.write_var_int(&(self.identifiers.len() as i32).into())?;

        for ident in self.identifiers {
            write.write_string(ident)?;
        }

        write.write_var_int(&(self.progress.len() as i32).into())?;

        for (adv_id, criteria) in self.progress {
            write.write_string(adv_id)?;
            let criterion_bytes = criterion_to_nbt(&criteria.identifier);
            write.write_slice(&criterion_bytes)?;
        }

        Ok(())
    }
}
