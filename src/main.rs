mod args;
mod cpu_state;
mod decode;
mod error;
mod instruction;
mod memory;
mod operand;
mod register;

use args::Args;
use clap::Parser;
use cpu_state::CpuState;
use decode::disassemble;
use memory::Address;

use std::fs::File;
use std::io::{BufReader, Read};

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mut file = File::open(args.file)?;

    if args.disassemble {
        let reader = BufReader::new(file);
        let bytes = reader
            .bytes()
            .enumerate()
            .filter_map(|(index, result)| match result {
                Ok(byte) => Some((Address(index as u16), byte)),
                Err(_) => None,
            });
        let disassembly = disassemble(&mut bytes.peekable())?;
        println!("{}", disassembly);
    } else {
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;
        let mut cpu_state = CpuState::new(&bytes);
        cpu_state.exec()?;
        cpu_state.print_registers();
    }

    Ok(())
}
