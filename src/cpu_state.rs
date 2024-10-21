use crate::decode::{decode_instruction, CountingPeekable};
use crate::instruction::Instruction;
use crate::memory::Address;
use crate::operand::Operand;
use crate::register::Register;
use anyhow::Result;

pub struct CpuState<'a> {
    instructions: &'a [u8],
    instruction_pointer: u16,
    registers: [u16; 8],
}

impl<'a> CpuState<'a> {
    pub fn new(instructions: &'a [u8]) -> Self {
        CpuState {
            instructions,
            instruction_pointer: 0,
            registers: [0; 8],
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
        println!(
            "Executing {} at 0x{:x}",
            instruction, self.instruction_pointer
        );

        self.instruction_pointer += instruction.get_size() as u16;

        match instruction {
            Instruction::Mov { dst, src, .. } => match dst {
                Operand::Register(reg) => {
                    let (index, mask) = self.get_register_index_and_mask(&reg);
                    self.registers[index] = self.get_operand_value(&src) & mask;
                }
                Operand::Memory(_) => todo!(),
                Operand::Immediate8(_) => unreachable!(),
                Operand::Immediate16(_) => unreachable!(),
            },
            Instruction::Add { .. } => todo!(),
            Instruction::Sub { .. } => todo!(),
            Instruction::Cmp { .. } => todo!(),
            Instruction::Je { .. } => todo!(),
            Instruction::Jl { .. } => todo!(),
            Instruction::Jle { .. } => todo!(),
            Instruction::Jb { .. } => todo!(),
            Instruction::Jbe { .. } => todo!(),
            Instruction::Jp { .. } => todo!(),
            Instruction::Jo { .. } => todo!(),
            Instruction::Js { .. } => todo!(),
            Instruction::Jne { .. } => todo!(),
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

        Ok(())
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
