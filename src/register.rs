use std::fmt::Formatter;

pub enum Register {
    Al = 0,
    Cl = 1,
    Dl = 2,
    Bl = 3,
    Ah = 4,
    Ch = 5,
    Dh = 6,
    Bh = 7,
    Ax = 8,
    Cx = 9,
    Dx = 10,
    Bx = 11,
    Sp = 12,
    Bp = 13,
    Si = 14,
    Di = 15,
}

impl std::fmt::Display for Register {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Register::Al => write!(f, "al"),
            Register::Cl => write!(f, "cl"),
            Register::Dl => write!(f, "dl"),
            Register::Bl => write!(f, "bl"),
            Register::Ah => write!(f, "ah"),
            Register::Ch => write!(f, "ch"),
            Register::Dh => write!(f, "dh"),
            Register::Bh => write!(f, "bh"),
            Register::Ax => write!(f, "ax"),
            Register::Cx => write!(f, "cx"),
            Register::Dx => write!(f, "dx"),
            Register::Bx => write!(f, "bx"),
            Register::Sp => write!(f, "sp"),
            Register::Bp => write!(f, "bp"),
            Register::Si => write!(f, "si"),
            Register::Di => write!(f, "di"),
        }
    }
}

impl Register {
    pub fn decode_reg(reg: u8, w: u8) -> Register {
        match (reg, w) {
            (0b0000, 0) => Register::Al,
            (0b0000, 1) => Register::Ax,
            (0b0001, 0) => Register::Cl,
            (0b0001, 1) => Register::Cx,
            (0b0010, 0) => Register::Dl,
            (0b0010, 1) => Register::Dx,
            (0b0011, 0) => Register::Bl,
            (0b0011, 1) => Register::Bx,
            (0b0100, 0) => Register::Ah,
            (0b0100, 1) => Register::Sp,
            (0b0101, 0) => Register::Ch,
            (0b0101, 1) => Register::Bp,
            (0b0110, 0) => Register::Dh,
            (0b0110, 1) => Register::Si,
            (0b0111, 0) => Register::Bh,
            (0b0111, 1) => Register::Di,
            _ => unreachable!(),
        }
    }

    pub fn effective_address_calculation(rm: u8) -> [Option<Register>; 2]
    {
        match rm & 0b111 {
            0b000 => [Some(Register::Bx), Some(Register::Si)],
            0b001 => [Some(Register::Bx), Some(Register::Di)],
            0b010 => [Some(Register::Bp), Some(Register::Si)],
            0b011 => [Some(Register::Bp), Some(Register::Di)],
            0b100 => [Some(Register::Si), None],
            0b101 => [Some(Register::Di), None],
            0b110 => if (rm & 0b11000000) == 0b00 { [None, None] } else { [Some(Register::Bp), None] },
            0b111 => [Some(Register::Bx), None],
            _ => unreachable!()
        }
    }
}
