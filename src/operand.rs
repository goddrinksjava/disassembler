use crate::decode::{AddressByteIteratorExt, CountingPeekable};
use crate::memory::Displacement::{Disp16, Disp8};
use crate::memory::{Address, Displacement, Memory};
use crate::register::Register;
use std::fmt::Formatter;

pub enum Operand {
    Register(Register),
    Memory(Memory),
    Immediate8(u8),
    Immediate16(u16),
}

impl Operand {
    pub fn immediate<T>(w: u8, bytes: &mut CountingPeekable<T>) -> anyhow::Result<[Operand; 2]>
    where
        T: Iterator<Item = (Address, u8)>,
    {
        let (_mod_rm_address, mod_rm) = bytes.try_next()?;
        let op_rm = Operand::from_mod_rm(w, mod_rm, bytes)?;

        if w == 0 {
            let (_address, data) = bytes.try_next()?;
            Ok([op_rm, Operand::Immediate8(data)])
        } else {
            let (_address, data_lo) = bytes.try_next()?;
            let (_address, data_hi) = bytes.try_next()?;
            Ok([
                op_rm,
                Operand::Immediate16(((data_hi as u16) << 8) | data_lo as u16),
            ])
        }
    }

    pub fn from_mod_rm<T>(
        w: u8,
        mod_rm: u8,
        bytes: &mut CountingPeekable<T>,
    ) -> anyhow::Result<Operand>
    where
        T: Iterator<Item = (Address, u8)>,
    {
        match mod_rm & 0b1100_0000 {
            0b0000_0000 => {
                let displacement = if (mod_rm & 0b0000_0111) == 0b0000_0110 {
                    let (_, byte1) = bytes.try_next()?;
                    let (_, byte2) = bytes.try_next()?;
                    Disp16(((byte2 as u16) << 8) | byte1 as u16)
                } else {
                    Displacement::None
                };

                let registers = Register::effective_address_calculation(mod_rm);
                Ok(Operand::Memory(Memory {
                    displacement,
                    registers,
                }))
            }
            0b0100_0000 => {
                let (_, byte1) = bytes.try_next()?;
                let displacement = Disp8(byte1);
                let registers = Register::effective_address_calculation(mod_rm);
                Ok(Operand::Memory(Memory {
                    displacement,
                    registers,
                }))
            }
            0b1000_0000 => {
                let (_, byte1) = bytes.try_next()?;
                let (_, byte2) = bytes.try_next()?;
                let displacement = Disp16(((byte2 as u16) << 8) | byte1 as u16);
                let registers = Register::effective_address_calculation(mod_rm);
                Ok(Operand::Memory(Memory {
                    displacement,
                    registers,
                }))
            }
            0b1100_0000 => Ok(Operand::Register(Register::decode_reg(
                mod_rm & 0b0000_0111,
                w,
            ))),
            _ => unreachable!(),
        }
    }

    pub fn from_mod_reg_rm<T>(
        d: u8,
        w: u8,
        bytes: &mut CountingPeekable<T>,
    ) -> anyhow::Result<[Operand; 2]>
    where
        T: Iterator<Item = (Address, u8)>,
    {
        let (_mod_reg_rm_address, mod_reg_rm) = bytes.try_next()?;
        let op_reg = Operand::Register(Register::decode_reg((mod_reg_rm & 0b0011_1000) >> 3, w));
        let op_rm = Operand::from_mod_rm(w, mod_reg_rm, bytes)?;

        if d == 0 {
            Ok([op_rm, op_reg])
        } else {
            Ok([op_reg, op_rm])
        }
    }
}

impl std::fmt::Display for Operand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::Register(reg) => {
                write!(f, "{reg}")
            }
            Operand::Memory(mem) => {
                write!(f, "{mem}")
            }
            Operand::Immediate8(imm) => {
                write!(f, "byte {imm}")
            }
            Operand::Immediate16(imm) => {
                write!(f, "word {imm}")
            }
        }
    }
}
