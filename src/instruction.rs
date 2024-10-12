use std::fmt::Formatter;
use crate::operand::Operand;

pub enum Instruction {
    Mov { dst: Operand, src: Operand },
    Add { dst: Operand, src: Operand },
    Sub { dst: Operand, src: Operand },
    Cmp { dst: Operand, src: Operand },
    Je { ip_increment: u8 },
    Jl { ip_increment: u8 },
    Jle { ip_increment: u8 },
    Jb { ip_increment: u8 },
    Jbe { ip_increment: u8 },
    Jp { ip_increment: u8 },
    Jo { ip_increment: u8 },
    Js { ip_increment: u8 },
    Jne { ip_increment: u8 },
    Jnl { ip_increment: u8 },
    Jnle { ip_increment: u8 },
    Jnb { ip_increment: u8 },
    Jnbe { ip_increment: u8 },
    Jnp { ip_increment: u8 },
    Jno { ip_increment: u8 },
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Mov { dst, src } => {
                write!(f, "mov {dst}, {src}")
            }
            Instruction::Add { dst, src } => {
                write!(f, "add {dst}, {src}")
            }
            Instruction::Sub { dst, src } => {
                write!(f, "sub {dst}, {src}")
            }
            Instruction::Cmp { dst, src } => {
                write!(f, "cmp {dst}, {src}")
            }
            Instruction::Je { ip_increment } => {
                write!(f, "je {ip_increment}")
            }
            Instruction::Jl { ip_increment } => {
                write!(f, "jl {ip_increment}")
            }
            Instruction::Jle { ip_increment } => {
                write!(f, "jle {ip_increment}")
            }
            Instruction::Jb { ip_increment } => {
                write!(f, "jb {ip_increment}")
            }
            Instruction::Jbe { ip_increment } => {
                write!(f, "jbe {ip_increment}")
            }
            Instruction::Jp { ip_increment } => {
                write!(f, "jp {ip_increment}")
            }
            Instruction::Jo { ip_increment } => {
                write!(f, "jo {ip_increment}")
            }
            Instruction::Js { ip_increment } => {
                write!(f, "js {ip_increment}")
            }
            Instruction::Jne { ip_increment } => {
                write!(f, "jne {ip_increment}")
            }
            Instruction::Jnl { ip_increment } => {
                write!(f, "jnl {ip_increment}")
            }
            Instruction::Jnle { ip_increment } => {
                write!(f, "jnle {ip_increment}")
            }
            Instruction::Jnb { ip_increment } => {
                write!(f, "jnb {ip_increment}")
            }
            Instruction::Jnbe { ip_increment } => {
                write!(f, "jnbe {ip_increment}")
            }
            Instruction::Jnp { ip_increment } => {
                write!(f, "jnp {ip_increment}")
            }
            Instruction::Jno { ip_increment } => {
                write!(f, "jno {ip_increment}")
            }
        }
    }
}

