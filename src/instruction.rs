use std::fmt::Formatter;
use crate::operand::Operand;

pub enum Instruction {
    Mov { dst: Operand, src: Operand },
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Mov { dst, src } => {
                write!(f, "mov {dst}, {src}")
            }
        }
    }
}

