use crate::memory::Address;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unknown instruction ({0:#b}) at {:#x}", .0)]
    UnknownInstruction(u8, Address),
    #[error("Unexpected end of instruction stream")]
    EndOfInstructionStream(),
}
