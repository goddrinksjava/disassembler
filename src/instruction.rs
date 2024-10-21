use crate::operand::Operand;
use std::fmt::Formatter;

pub enum Instruction {
    Mov { sz: u8, dst: Operand, src: Operand },
    Add { sz: u8, dst: Operand, src: Operand },
    Sub { sz: u8, dst: Operand, src: Operand },
    Cmp { sz: u8, dst: Operand, src: Operand },
    Je { sz: u8, ip_increment: u8 },
    Jl { sz: u8, ip_increment: u8 },
    Jle { sz: u8, ip_increment: u8 },
    Jb { sz: u8, ip_increment: u8 },
    Jbe { sz: u8, ip_increment: u8 },
    Jp { sz: u8, ip_increment: u8 },
    Jo { sz: u8, ip_increment: u8 },
    Js { sz: u8, ip_increment: u8 },
    Jne { sz: u8, ip_increment: u8 },
    Jnl { sz: u8, ip_increment: u8 },
    Jnle { sz: u8, ip_increment: u8 },
    Jnb { sz: u8, ip_increment: u8 },
    Jnbe { sz: u8, ip_increment: u8 },
    Jnp { sz: u8, ip_increment: u8 },
    Jno { sz: u8, ip_increment: u8 },
    Jns { sz: u8, ip_increment: u8 },
    Loop { sz: u8, ip_increment: u8 },
    Loopz { sz: u8, ip_increment: u8 },
    Loopnz { sz: u8, ip_increment: u8 },
    Jcxz { sz: u8, ip_increment: u8 },
}

impl Instruction {
    pub fn get_size(&self) -> u8 {
        match self {
            Instruction::Mov { sz, .. }
            | Instruction::Add { sz, .. }
            | Instruction::Sub { sz, .. }
            | Instruction::Cmp { sz, .. }
            | Instruction::Je { sz, .. }
            | Instruction::Jl { sz, .. }
            | Instruction::Jle { sz, .. }
            | Instruction::Jb { sz, .. }
            | Instruction::Jbe { sz, .. }
            | Instruction::Jp { sz, .. }
            | Instruction::Jo { sz, .. }
            | Instruction::Js { sz, .. }
            | Instruction::Jne { sz, .. }
            | Instruction::Jnl { sz, .. }
            | Instruction::Jnle { sz, .. }
            | Instruction::Jnb { sz, .. }
            | Instruction::Jnbe { sz, .. }
            | Instruction::Jnp { sz, .. }
            | Instruction::Jno { sz, .. }
            | Instruction::Jns { sz, .. }
            | Instruction::Loop { sz, .. }
            | Instruction::Loopz { sz, .. }
            | Instruction::Loopnz { sz, .. }
            | Instruction::Jcxz { sz, .. } => *sz,
        }
    }

    pub fn to_jump(&self) -> Option<Jump> {
        match self {
            Instruction::Je { ip_increment, .. } => Some(Jump::Je {
                ip_increment: (*ip_increment as i8) as i16,
            }),
            Instruction::Jl { ip_increment, .. } => Some(Jump::Jl {
                ip_increment: (*ip_increment as i8) as i16,
            }),
            Instruction::Jle { ip_increment, .. } => Some(Jump::Jle {
                ip_increment: (*ip_increment as i8) as i16,
            }),
            Instruction::Jb { ip_increment, .. } => Some(Jump::Jb {
                ip_increment: (*ip_increment as i8) as i16,
            }),
            Instruction::Jbe { ip_increment, .. } => Some(Jump::Jbe {
                ip_increment: (*ip_increment as i8) as i16,
            }),
            Instruction::Jp { ip_increment, .. } => Some(Jump::Jp {
                ip_increment: (*ip_increment as i8) as i16,
            }),
            Instruction::Jo { ip_increment, .. } => Some(Jump::Jo {
                ip_increment: (*ip_increment as i8) as i16,
            }),
            Instruction::Js { ip_increment, .. } => Some(Jump::Js {
                ip_increment: (*ip_increment as i8) as i16,
            }),
            Instruction::Jne { ip_increment, .. } => Some(Jump::Jne {
                ip_increment: (*ip_increment as i8) as i16,
            }),
            Instruction::Jnl { ip_increment, .. } => Some(Jump::Jnl {
                ip_increment: (*ip_increment as i8) as i16,
            }),
            Instruction::Jnle { ip_increment, .. } => Some(Jump::Jnle {
                ip_increment: (*ip_increment as i8) as i16,
            }),
            Instruction::Jnb { ip_increment, .. } => Some(Jump::Jnb {
                ip_increment: (*ip_increment as i8) as i16,
            }),
            Instruction::Jnbe { ip_increment, .. } => Some(Jump::Jnbe {
                ip_increment: (*ip_increment as i8) as i16,
            }),
            Instruction::Jnp { ip_increment, .. } => Some(Jump::Jnp {
                ip_increment: (*ip_increment as i8) as i16,
            }),
            Instruction::Jno { ip_increment, .. } => Some(Jump::Jno {
                ip_increment: (*ip_increment as i8) as i16,
            }),
            Instruction::Jns { ip_increment, .. } => Some(Jump::Jns {
                ip_increment: (*ip_increment as i8) as i16,
            }),
            Instruction::Loop { ip_increment, .. } => Some(Jump::Loop {
                ip_increment: (*ip_increment as i8) as i16,
            }),
            Instruction::Loopz { ip_increment, .. } => Some(Jump::Loopz {
                ip_increment: (*ip_increment as i8) as i16,
            }),
            Instruction::Loopnz { ip_increment, .. } => Some(Jump::Loopnz {
                ip_increment: (*ip_increment as i8) as i16,
            }),
            Instruction::Jcxz { ip_increment, .. } => Some(Jump::Jcxz {
                ip_increment: (*ip_increment as i8) as i16,
            }),
            _ => None,
        }
    }
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Mov { dst, src, .. } => {
                write!(f, "mov {dst}, {src}")
            }
            Instruction::Add { dst, src, .. } => {
                write!(f, "add {dst}, {src}")
            }
            Instruction::Sub { dst, src, .. } => {
                write!(f, "sub {dst}, {src}")
            }
            Instruction::Cmp { dst, src, .. } => {
                write!(f, "cmp {dst}, {src}")
            }
            Instruction::Je { ip_increment, .. } => {
                write!(f, "je {ip_increment}")
            }
            Instruction::Jl { ip_increment, .. } => {
                write!(f, "jl {ip_increment}")
            }
            Instruction::Jle { ip_increment, .. } => {
                write!(f, "jle {ip_increment}")
            }
            Instruction::Jb { ip_increment, .. } => {
                write!(f, "jb {ip_increment}")
            }
            Instruction::Jbe { ip_increment, .. } => {
                write!(f, "jbe {ip_increment}")
            }
            Instruction::Jp { ip_increment, .. } => {
                write!(f, "jp {ip_increment}")
            }
            Instruction::Jo { ip_increment, .. } => {
                write!(f, "jo {ip_increment}")
            }
            Instruction::Js { ip_increment, .. } => {
                write!(f, "js {ip_increment}")
            }
            Instruction::Jne { ip_increment, .. } => {
                write!(f, "jne {ip_increment}")
            }
            Instruction::Jnl { ip_increment, .. } => {
                write!(f, "jnl {ip_increment}")
            }
            Instruction::Jnle { ip_increment, .. } => {
                write!(f, "jnle {ip_increment}")
            }
            Instruction::Jnb { ip_increment, .. } => {
                write!(f, "jnb {ip_increment}")
            }
            Instruction::Jnbe { ip_increment, .. } => {
                write!(f, "jnbe {ip_increment}")
            }
            Instruction::Jnp { ip_increment, .. } => {
                write!(f, "jnp {ip_increment}")
            }
            Instruction::Jno { ip_increment, .. } => {
                write!(f, "jno {ip_increment}")
            }
            Instruction::Jns { ip_increment, .. } => {
                write!(f, "jns {ip_increment}")
            }
            Instruction::Loop { ip_increment, .. } => {
                write!(f, "loop {ip_increment}")
            }
            Instruction::Loopz { ip_increment, .. } => {
                write!(f, "loopz {ip_increment}")
            }
            Instruction::Loopnz { ip_increment, .. } => {
                write!(f, "loopnz {ip_increment}")
            }
            Instruction::Jcxz { ip_increment, .. } => {
                write!(f, "jcxz {ip_increment}")
            }
        }
    }
}

pub enum Jump {
    Je { ip_increment: i16 },
    Jl { ip_increment: i16 },
    Jle { ip_increment: i16 },
    Jb { ip_increment: i16 },
    Jbe { ip_increment: i16 },
    Jp { ip_increment: i16 },
    Jo { ip_increment: i16 },
    Js { ip_increment: i16 },
    Jne { ip_increment: i16 },
    Jnl { ip_increment: i16 },
    Jnle { ip_increment: i16 },
    Jnb { ip_increment: i16 },
    Jnbe { ip_increment: i16 },
    Jnp { ip_increment: i16 },
    Jno { ip_increment: i16 },
    Jns { ip_increment: i16 },
    Loop { ip_increment: i16 },
    Loopz { ip_increment: i16 },
    Loopnz { ip_increment: i16 },
    Jcxz { ip_increment: i16 },
}

impl Jump {
    pub fn ip_increment(&self) -> i16 {
        match self {
            Jump::Je { ip_increment }
            | Jump::Jl { ip_increment }
            | Jump::Jle { ip_increment }
            | Jump::Jb { ip_increment }
            | Jump::Jbe { ip_increment }
            | Jump::Jp { ip_increment }
            | Jump::Jo { ip_increment }
            | Jump::Js { ip_increment }
            | Jump::Jne { ip_increment }
            | Jump::Jnl { ip_increment }
            | Jump::Jnle { ip_increment }
            | Jump::Jnb { ip_increment }
            | Jump::Jnbe { ip_increment }
            | Jump::Jnp { ip_increment }
            | Jump::Jno { ip_increment }
            | Jump::Jns { ip_increment }
            | Jump::Loop { ip_increment }
            | Jump::Loopz { ip_increment }
            | Jump::Loopnz { ip_increment }
            | Jump::Jcxz { ip_increment } => *ip_increment,
        }
    }

    pub fn len(&self) -> i16 {
        2
    }
}

impl std::fmt::Display for Jump {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Jump::Je { .. } => {
                write!(f, "je")
            }
            Jump::Jl { .. } => {
                write!(f, "jl")
            }
            Jump::Jle { .. } => {
                write!(f, "jle")
            }
            Jump::Jb { .. } => {
                write!(f, "jb")
            }
            Jump::Jbe { .. } => {
                write!(f, "jbe")
            }
            Jump::Jp { .. } => {
                write!(f, "jp")
            }
            Jump::Jo { .. } => {
                write!(f, "jo")
            }
            Jump::Js { .. } => {
                write!(f, "js")
            }
            Jump::Jne { .. } => {
                write!(f, "jne")
            }
            Jump::Jnl { .. } => {
                write!(f, "jnl")
            }
            Jump::Jnle { .. } => {
                write!(f, "jnle")
            }
            Jump::Jnb { .. } => {
                write!(f, "jnb")
            }
            Jump::Jnbe { .. } => {
                write!(f, "jnbe")
            }
            Jump::Jnp { .. } => {
                write!(f, "jnp")
            }
            Jump::Jno { .. } => {
                write!(f, "jno")
            }
            Jump::Jns { .. } => {
                write!(f, "jns")
            }
            Jump::Loop { .. } => {
                write!(f, "loop")
            }
            Jump::Loopz { .. } => {
                write!(f, "loopz")
            }
            Jump::Loopnz { .. } => {
                write!(f, "loopnz")
            }
            Jump::Jcxz { .. } => {
                write!(f, "jcxz")
            }
        }
    }
}

