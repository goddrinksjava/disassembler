use std::fmt::Formatter;
use crate::register::Register;

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct Address(pub u16);

pub struct Memory {
    pub displacement: Displacement,
    pub registers: [Option<Register>; 2],
}

pub enum Displacement {
    Disp8(u8),
    Disp16(u16),
    None,
}

impl std::fmt::Display for Memory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;

        let mut s = String::new();
        let mut flag = false;

        if let Some(reg) = &self.registers[0]
        {
            s.push_str(&reg.to_string());
            flag = true;
        }
        if let Some(reg) = &self.registers[1]
        {
            if flag {
                s.push_str(" + ");
            }
            s.push_str(&reg.to_string());
            flag = true;
        }
        match self.displacement {
            Displacement::Disp8(disp) => {
                let disp = disp as i8;
                if flag {
                    if disp >= 0 {
                        s.push_str(" + ");
                    } else {
                        s.push_str(" - ");
                    }
                }
                s.push_str(&(disp as i32).abs().to_string());
            }
            Displacement::Disp16(disp) => {
                let disp = disp as i16;
                if flag {
                    if disp >= 0 {
                        s.push_str(" + ");
                    } else {
                        s.push_str(" - ");
                    }
                }
                s.push_str(&(disp as i32).abs().to_string());
            }
            Displacement::None => {}
        }

        write!(f, "{}", s)?;

        write!(f, "]")
    }
}
