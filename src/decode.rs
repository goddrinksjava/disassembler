use std::iter::Peekable;
use anyhow::Result;

use crate::{instruction::Instruction, memory::Address};
use crate::memory::{Displacement, Memory};
use crate::memory::Displacement::{Disp16, Disp8};
use crate::operand::Operand;
use crate::register::Register;

pub fn disassemble<T>(bytes: &mut Peekable<T>) -> Result<String>
where
    T: Iterator<Item=(Address, u8)>,
{
    let mut disassembly = String::new();
    disassembly.push_str("bits 16\n\n");

    while bytes.peek().is_some() {
        let instruction = decode_instruction(bytes)?;
        disassembly.push_str(&(instruction.to_string() + "\n"))
    }
    Ok(disassembly)
}

macro_rules! decode_arithmetic_binop_register_to_either {
    ($bytes:expr, $variant:ident) => {{
        let (_, byte1) = $bytes.try_next()?;
        let d = (byte1 & 0b00000010) >> 1;
        let w = byte1 & 0b00000001;

        let [dst, src] = Operand::from_mod_reg_rm(d, w, $bytes)?;

        Ok(Instruction::$variant { dst, src })
    }};
}

macro_rules! decode_arithmetic_binop_immediate_to_register_memory {
    ($bytes:expr, { $($pattern:expr => $variant:ident),+ $(,)? }) => {{
        let (_, byte1) = $bytes.try_next()?;
        let (_, mod_rm) = $bytes.try_next()?;

        let s = (byte1 & 0b00000010) >> 1;
        let w = byte1 & 0b00000001;

        let dst = Operand::from_mod_rm(w, mod_rm, $bytes)?;

        let src = if w == 0
        {
            let (_address, data) = $bytes.try_next()?;
            Operand::Immediate8(data)
        } else if s == 0 {
            let (_address, data_lo) = $bytes.try_next()?;
            let (_address, data_hi) = $bytes.try_next()?;
            Operand::Immediate16(((data_hi as u16) << 8) | data_lo as u16)
        } else {
            let (_address, data_lo) = $bytes.try_next()?;
            Operand::Immediate16(((0u16) << 8) | data_lo as u16)
        };

        match (mod_rm & 0b00111000) {
            $(
                $pattern => Ok(Instruction::$variant { dst, src }),
            )*
            _ => unreachable!()
        }
    }};
}

macro_rules! decode_arithmetic_binop_immediate_to_accumulator {
    ($bytes:expr, $variant:ident) => {{
        let (_, byte1) = $bytes.try_next()?;
        let w = byte1 & 0b00000001;

        let dst =
            if w == 0 { Operand::Register(Register::Al) } else { Operand::Register(Register::Ax) };

        let src = if w == 0
        {
            let (_address, data) = $bytes.try_next()?;
            Operand::Immediate8(data)
        } else {
            let (_address, data_lo) = $bytes.try_next()?;
            let (_address, data_hi) = $bytes.try_next()?;
            Operand::Immediate16(((data_hi as u16) << 8) | data_lo as u16)
        };

        Ok(Instruction::$variant { dst, src })
    }};
}

macro_rules! decode_jump {
    ($bytes:expr, $variant:ident) => {{
        $bytes.try_next()?;
        let (_, ip_increment) = $bytes.try_next()?;

        Ok(Instruction::$variant { ip_increment })
    }};
}

pub fn decode_instruction<T>(bytes: &mut Peekable<T>) -> Result<Instruction>
where
    T: Iterator<Item=(Address, u8)>,
{
    let (address, byte) = bytes
        .peek()
        .cloned()
        .ok_or(crate::error::Error::EndOfInstructionStream())?;

    match byte {
        0b1000_1000..=0b1000_1011 => decode_mov_register_memory(bytes),
        0b1011_0000..=0b1011_1111 => decode_mov_immediate_to_reg(bytes),
        0b1100_0110..=0b1100_0111 => decode_mov_immediate_to_reg_mem(bytes),
        0b1010_0000..=0b1010_0001 => decode_mem_to_accumulator(bytes),
        0b1010_0010..=0b1010_0011 => decode_accumulator_to_mem(bytes),

        0b0000_0000..=0b0000_0011 => decode_arithmetic_binop_register_to_either!(bytes, Add),
        0b0010_1000..=0b0010_1011 => decode_arithmetic_binop_register_to_either!(bytes, Sub),
        0b0011_1000..=0b0011_1011 => decode_arithmetic_binop_register_to_either!(bytes, Cmp),
        0b1000_0000..=0b1000_0011 => decode_arithmetic_binop_immediate_to_register_memory!(
            bytes,
            {
                0b0000_0000 => Add,
                0b0010_1000 => Sub,
                0b0011_1000 => Cmp,
            }
        ),
        0b0000_0100..=0b0000_0111 => decode_arithmetic_binop_immediate_to_accumulator!(bytes, Add),
        0b0010_1100..=0b0010_1111 => decode_arithmetic_binop_immediate_to_accumulator!(bytes, Sub),
        0b0011_1100..=0b0011_1111 => decode_arithmetic_binop_immediate_to_accumulator!(bytes, Cmp),

        0b0111_0100 => decode_jump!(bytes, Je),
        0b0111_1100 => decode_jump!(bytes, Jl),
        0b0111_1110 => decode_jump!(bytes, Jle),
        0b0111_0010 => decode_jump!(bytes, Jb),
        0b0111_0110 => decode_jump!(bytes, Jbe),
        0b0111_1010 => decode_jump!(bytes, Jp),
        0b0111_0000 => decode_jump!(bytes, Jo),
        0b0111_1000 => decode_jump!(bytes, Js),
        0b0111_0101 => decode_jump!(bytes, Jne),
        0b0111_1101 => decode_jump!(bytes, Jnl),
        0b0111_1111 => decode_jump!(bytes, Jnle),
        0b0111_0011 => decode_jump!(bytes, Jnb),
        0b0111_0111 => decode_jump!(bytes, Jnbe),
        0b0111_1011 => decode_jump!(bytes, Jnp),
        0b0111_0001 => decode_jump!(bytes, Jno),

        _ => Err(crate::error::Error::UnknownInstruction(byte, address).into()),
    }
}

fn decode_mov_immediate_to_reg_mem<T>(bytes: &mut Peekable<T>) -> Result<Instruction>
where
    T: Iterator<Item=(Address, u8)>,
{
    let (_address1, byte1) = bytes.try_next()?;
    let w = byte1 & 0b00000001;

    let [dst, src] = Operand::immediate(w, bytes)?;
    Ok(Instruction::Mov {
        dst,
        src,
    })
}

fn decode_mov_immediate_to_reg<T>(bytes: &mut Peekable<T>) -> Result<Instruction>
where
    T: Iterator<Item=(Address, u8)>,
{
    let (_address1, byte1) = bytes.try_next()?;

    let w = (byte1 & 0b00001000) >> 3;
    let reg = byte1 & 0b00000111;

    let (_address2, byte2) = bytes.try_next()?;

    if w == 0 {
        Ok(Instruction::Mov {
            dst: Operand::Register(Register::decode_reg(reg, w)),
            src: Operand::Immediate8(byte2),
        })
    } else {
        let (_address3, byte3) = bytes.try_next()?;

        Ok(Instruction::Mov {
            dst: Operand::Register(Register::decode_reg(reg, w)),
            src: Operand::Immediate16(((byte3 as u16) << 8) | byte2 as u16),
        })
    }
}

fn decode_mov_register_memory<T>(bytes: &mut Peekable<T>) -> Result<Instruction>
where
    T: Iterator<Item=(Address, u8)>,
{
    let (_address1, byte1) = bytes.try_next()?;

    let d = (byte1 & 0b00000010) >> 1;
    let w = byte1 & 0b00000001;

    let [dst, src] = Operand::from_mod_reg_rm(d, w, bytes)?;

    Ok(Instruction::Mov { dst, src })
}

fn decode_mem_to_accumulator<T>(bytes: &mut Peekable<T>) -> Result<Instruction>
where
    T: Iterator<Item=(Address, u8)>,
{
    let (reg, displacement) = decode_accumulator_and_mem(bytes)?;

    Ok(Instruction::Mov {
        dst: Operand::Register(reg),
        src: Operand::Memory(Memory { displacement, registers: [None, None] }),
    })
}

fn decode_accumulator_to_mem<T>(bytes: &mut Peekable<T>) -> Result<Instruction>
where
    T: Iterator<Item=(Address, u8)>,
{
    let (reg, displacement) = decode_accumulator_and_mem(bytes)?;

    Ok(Instruction::Mov {
        dst: Operand::Memory(Memory { displacement, registers: [None, None] }),
        src: Operand::Register(reg),
    })
}


fn decode_accumulator_and_mem<T>(bytes: &mut Peekable<T>) -> Result<(Register, Displacement)>
where
    T: Iterator<Item=(Address, u8)>,
{
    let (_address1, byte1) = bytes.try_next()?;
    let w = byte1 & 0b00000001;

    if w == 0
    {
        let (_address, data) = bytes.try_next()?;
        Ok((Register::Al, Disp8(data)))
    } else {
        let (_address, data_lo) = bytes.try_next()?;
        let (_address, data_hi) = bytes.try_next()?;
        Ok(
            (Register::Ax, Disp16(((data_hi as u16) << 8) | data_lo as u16))
        )
    }
}

pub trait AddressByteIteratorExt: Iterator<Item=(Address, u8)> {
    fn try_next(&mut self) -> Result<(Address, u8), crate::error::Error>;
}

impl<I> AddressByteIteratorExt for I
where
    I: Iterator<Item=(Address, u8)>,
{
    fn try_next(&mut self) -> Result<(Address, u8), crate::error::Error> {
        self.next()
            .ok_or(crate::error::Error::EndOfInstructionStream())
    }
}

#[cfg(test)]
mod tests {
    use crate::decode::disassemble;
    use crate::memory::Address;
    use std::fs::File;
    use std::io::{self, BufReader, Read, Write};
    use std::process::Command;
    use std::{env, fs};

    #[test]
    fn test_disassemble() -> anyhow::Result<()> {
        let asm_files = get_test_file_paths()?;

        for asm_file in asm_files {
            let original_bin = assemble_file(&asm_file)?;
            let disassembled_output = match disassemble_binary(&original_bin) {
                Ok(it) => it,
                Err(e) => {
                    eprintln!("{}", e);
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        format!("Error disassembling {}", asm_file),
                    )
                        .into());
                }
            };
            let reassembled_bin = match assemble_from_string(&disassembled_output) {
                Ok(it) => it,
                Err(e) => {
                    eprintln!("{}", e);
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        format!("Error assembling {}", asm_file),
                    )
                        .into());
                }
            };

            if original_bin != reassembled_bin {
                eprintln!("disassembly:\n{}", disassembled_output);
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    format!("Binaries do not match for file {}", asm_file),
                )
                    .into());
            }
        }

        Ok(())
    }

    fn get_test_file_paths() -> io::Result<Vec<String>> {
        let mut file_paths = Vec::new();

        for entry in fs::read_dir("res")? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                file_paths.push(path.display().to_string());
            }
        }

        Ok(file_paths)
    }

    fn assemble_file(asm_file: &str) -> io::Result<Vec<u8>> {
        let output_file = asm_file.to_owned() + ".bin";
        let output = Command::new("nasm")
            .args(["-f", "bin", asm_file, "-o", &output_file])
            .output()?;

        if output.status.success() {
            let data = fs::read(&output_file)?;
            fs::remove_file(&output_file)?;
            Ok(data)
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "Failed to assemble file: {}",
                    String::from_utf8_lossy(&output.stderr)
                ),
            ))
        }
    }

    fn assemble_from_string(asm_content: &str) -> io::Result<Vec<u8>> {
        let temp_dir = env::temp_dir();
        let temp_file_path = temp_dir.join("temp.asm");
        let out_file_path = temp_dir.join("temp.asm.bin");

        {
            let mut temp_file = File::create(&temp_file_path)?;
            temp_file.write_all(asm_content.as_bytes())?;
        }

        let output = Command::new("nasm")
            .args([
                "-f",
                "bin",
                temp_file_path.to_str().unwrap(),
                "-o",
                out_file_path.to_str().unwrap(),
            ])
            .output()?;

        if output.status.success() {
            let data = fs::read(&out_file_path)?;
            fs::remove_file(&temp_file_path)?;
            fs::remove_file(&out_file_path)?;
            Ok(data)
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "Failed to assemble from string: {}",
                    String::from_utf8_lossy(&output.stderr)
                ),
            ))
        }
    }

    fn disassemble_binary(bin_content: &[u8]) -> anyhow::Result<String> {
        let reader = BufReader::new(bin_content);
        let bytes = reader.bytes();

        let disassemble_bytes = bytes
            .enumerate()
            .filter_map(|(index, result)| match result {
                Ok(byte) => Some((Address(index as u16), byte)),
                Err(_) => None,
            });
        let disassembly = disassemble(&mut disassemble_bytes.peekable())?;

        Ok(disassembly)
    }
}
