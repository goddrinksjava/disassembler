mod decode;
mod error;
mod instruction;
mod memory;
mod register;
mod operand;

use crate::decode::disassemble;
use std::fs::File;
use std::io::{BufReader, Read};
use crate::memory::Address;

fn main() -> anyhow::Result<()> {
    let f = File::open("res/listing_0039_more_movs")?;
    let reader = BufReader::new(f);
    let bytes = reader.bytes().enumerate().filter_map(|(index, result)| {
        match result {
            Ok(byte) => Some((Address(index as u16), byte)),
            Err(_) => None
        }
    });
    let disassembly = disassemble(&mut bytes.peekable())?;
    println!("{}", disassembly);
    Ok(())
}
