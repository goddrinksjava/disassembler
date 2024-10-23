use std::fmt;

use crate::decode::{decode_instruction, CountingPeekable};
use crate::instruction::Instruction;
use crate::memory::Address;
use crate::operand::Operand;
use crate::register::Register;
use anyhow::Result;

#[derive(Clone, PartialEq, Eq)]
pub struct CpuStateFlags {
    flags: u8,
}

impl CpuStateFlags {
    const ZERO_FLAG_MASK: u8 = 0b0000_0001;
    const SIGN_FLAG_MASK: u8 = 0b0000_0010;

    pub fn new() -> CpuStateFlags {
        CpuStateFlags { flags: 0 }
    }

    pub fn set_zero_flag(&mut self, v: bool) {
        if v {
            self.flags |= Self::ZERO_FLAG_MASK;
        } else {
            self.flags &= !Self::ZERO_FLAG_MASK;
        }
    }

    pub fn get_zero_flag(&self) -> bool {
        (self.flags & Self::ZERO_FLAG_MASK) != 0
    }

    pub fn set_sign_flag(&mut self, v: bool) {
        if v {
            self.flags |= Self::SIGN_FLAG_MASK;
        } else {
            self.flags &= !Self::SIGN_FLAG_MASK;
        }
    }

    pub fn get_sign_flag(&self) -> bool {
        (self.flags & Self::SIGN_FLAG_MASK) != 0
    }
}

impl fmt::Display for CpuStateFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut flags_str = String::new();
        if self.flags & Self::ZERO_FLAG_MASK != 0 {
            flags_str.push('Z');
        }
        if self.flags & Self::SIGN_FLAG_MASK != 0 {
            flags_str.push('S');
        }
        write!(f, "{}", flags_str)
    }
}

pub struct CpuState<'a> {
    instructions: &'a [u8],
    instruction_pointer: u16,
    registers: [u16; 8],
    flags: CpuStateFlags,
}

impl<'a> CpuState<'a> {
    pub fn new(instructions: &'a [u8]) -> Self {
        CpuState {
            instructions,
            instruction_pointer: 0,
            registers: [0; 8],
            flags: CpuStateFlags::new(),
        }
    }

    pub fn print_registers(&self) {
        println!("Registers:");
        println!("AX: {:04X}", self.registers[0]);
        println!("BX: {:04X}", self.registers[1]);
        println!("CX: {:04X}", self.registers[2]);
        println!("DX: {:04X}", self.registers[3]);
        println!("SP: {:04X}", self.registers[4]);
        println!("BP: {:04X}", self.registers[5]);
        println!("SI: {:04X}", self.registers[6]);
        println!("DI: {:04X}", self.registers[7]);
    }

    pub fn exec(&mut self) -> Result<()> {
        while self.instruction_pointer < self.instructions.len() as u16 {
            self.run_next_instruction()?;
        }

        Ok(())
    }

    fn decode_instruction(&mut self) -> Result<Instruction> {
        let instruction_iterator = self.instructions[self.instruction_pointer as usize..]
            .iter()
            .enumerate()
            .map(|(i, v)| (Address(i as u16), *v));

        decode_instruction(&mut CountingPeekable::new(instruction_iterator))
    }

    fn run_next_instruction(&mut self) -> Result<()> {
        let instruction = self.decode_instruction()?;
        print!(
            "Executing {} at 0x{:x}",
            instruction, self.instruction_pointer
        );

        let previous_instruction_pointer = self.instruction_pointer;
        self.instruction_pointer += instruction.get_size() as u16;

        match instruction {
            Instruction::Mov { dst, src, .. } => match dst {
                Operand::Register(reg) => {
                    let previous_flags = self.flags.clone();

                    let (index, mask) = self.get_register_index_and_mask(&reg);
                    let previous_register_value = self.registers[index] & mask;
                    self.registers[index] = self.get_operand_value(&src) & mask;
                    print!(
                        "; {}: 0x{:x} -> 0x{:x}",
                        reg,
                        previous_register_value,
                        self.registers[index] & mask
                    );
                    if previous_flags != self.flags {
                        print!("; flags:{} -> flags:{}", previous_flags, self.flags);
                    }
                }
                Operand::Memory(_) => todo!(),
                Operand::Immediate8(_) => unreachable!(),
                Operand::Immediate16(_) => unreachable!(),
            },
            Instruction::Add { dst, src, .. } => match dst {
                Operand::Register(reg) => {
                    let previous_flags = self.flags.clone();

                    let (index, mask) = self.get_register_index_and_mask(&reg);
                    let previous_register_value = self.registers[index] & mask;
                    let src_value = self.get_operand_value(&src);
                    let new_register_value = self.add_to_register(&reg, src_value);
                    self.registers[index] = new_register_value;

                    print!(
                        "; {}: 0x{:x} -> 0x{:x}",
                        reg,
                        previous_register_value,
                        self.registers[index] & mask
                    );

                    if previous_flags != self.flags {
                        print!("; flags:{} -> flags:{}", previous_flags, self.flags);
                    }
                }
                Operand::Memory(_) => todo!(),
                Operand::Immediate8(_) => unreachable!(),
                Operand::Immediate16(_) => unreachable!(),
            },
            Instruction::Sub { dst, src, .. } => match dst {
                Operand::Register(reg) => {
                    let previous_flags = self.flags.clone();

                    let (index, mask) = self.get_register_index_and_mask(&reg);
                    let previous_register_value = self.registers[index] & mask;
                    let src_value = self.get_operand_value(&src);
                    let new_register_value = self.add_to_register(&reg, src_value.wrapping_neg());
                    self.registers[index] = new_register_value;

                    print!(
                        "; {}: 0x{:x} -> 0x{:x}",
                        reg,
                        previous_register_value,
                        self.registers[index] & mask
                    );

                    if previous_flags != self.flags {
                        print!("; flags:{} -> flags:{}", previous_flags, self.flags);
                    }
                }
                Operand::Memory(_) => todo!(),
                Operand::Immediate8(_) => unreachable!(),
                Operand::Immediate16(_) => unreachable!(),
            },
            Instruction::Cmp { dst, src, .. } => match dst {
                Operand::Register(reg) => {
                    let previous_flags = self.flags.clone();

                    let src_value = self.get_operand_value(&src);
                    self.add_to_register(&reg, src_value.wrapping_neg());

                    if previous_flags != self.flags {
                        print!("; flags:{} -> flags:{}", previous_flags, self.flags);
                    }
                }
                Operand::Memory(_) => todo!(),
                Operand::Immediate8(_) => unreachable!(),
                Operand::Immediate16(_) => unreachable!(),
            },
            Instruction::Je { .. } => todo!(),
            Instruction::Jl { .. } => todo!(),
            Instruction::Jle { .. } => todo!(),
            Instruction::Jb { .. } => todo!(),
            Instruction::Jbe { .. } => todo!(),
            Instruction::Jp { .. } => todo!(),
            Instruction::Jo { .. } => todo!(),
            Instruction::Js { .. } => todo!(),
            Instruction::Jne { ip_increment, .. } => {
                if self.flags.get_zero_flag() == false {
                    self.instruction_pointer =
                        self.instruction_pointer.wrapping_add(ip_increment as u16);
                }
            }
            Instruction::Jnl { .. } => todo!(),
            Instruction::Jnle { .. } => todo!(),
            Instruction::Jnb { .. } => todo!(),
            Instruction::Jnbe { .. } => todo!(),
            Instruction::Jnp { .. } => todo!(),
            Instruction::Jno { .. } => todo!(),
            Instruction::Jns { .. } => todo!(),
            Instruction::Loop { .. } => todo!(),
            Instruction::Loopz { .. } => todo!(),
            Instruction::Loopnz { .. } => todo!(),
            Instruction::Jcxz { .. } => todo!(),
        }

        println!(
            "; ip:0x{:x} -> ip:0x{:x}",
            previous_instruction_pointer, self.instruction_pointer
        );
        Ok(())
    }

    fn add_to_register(&mut self, dst: &Register, src_value: u16) -> u16 {
        let (index, mask) = self.get_register_index_and_mask(&dst);

        if mask == 0xFFFF {
            let value = (self.registers[index]).wrapping_add(src_value);

            self.flags.set_zero_flag(value == 0);
            self.flags
                .set_sign_flag((value & 0b1000_0000_0000_0000) != 0);

            value
        } else {
            let src_value = if mask == 0xFF00 {
                ((src_value & mask) >> 8) as u8
            } else {
                (src_value & mask) as u8
            };

            let dst_value = if mask == 0xFF00 {
                ((self.registers[index] & mask) >> 8) as u8
            } else {
                (self.registers[index] & mask) as u8
            };

            let value = dst_value.wrapping_add(src_value);

            self.flags.set_zero_flag(value == 0);
            self.flags.set_sign_flag((value & 0b1000_0000) != 0);

            (value as u16 & mask) & (self.registers[index] & mask.swap_bytes())
        }
    }

    fn get_operand_value(&self, op: &Operand) -> u16 {
        match op {
            Operand::Register(reg) => {
                let (index, mask) = self.get_register_index_and_mask(reg);
                self.registers[index] & mask
            }
            Operand::Memory(_) => todo!(),
            Operand::Immediate8(v) => *v as u16,
            Operand::Immediate16(v) => *v,
        }
    }

    fn get_register_index_and_mask(&self, reg: &Register) -> (usize, u16) {
        match reg {
            Register::Al => (0, 0x00FF),
            Register::Ah => (0, 0xFF00),
            Register::Ax => (0, 0xFFFF),
            Register::Bl => (1, 0x00FF),
            Register::Bh => (1, 0xFF00),
            Register::Bx => (1, 0xFFFF),
            Register::Cl => (2, 0x00FF),
            Register::Ch => (2, 0xFF00),
            Register::Cx => (2, 0xFFFF),
            Register::Dl => (3, 0x00FF),
            Register::Dh => (3, 0xFF00),
            Register::Dx => (3, 0xFFFF),
            Register::Sp => (4, 0xFFFF),
            Register::Bp => (5, 0xFFFF),
            Register::Si => (6, 0xFFFF),
            Register::Di => (7, 0xFFFF),
        }
    }
}
