use clap::{arg, ArgGroup, Parser};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[clap(group(
    ArgGroup::new("mode")
        .required(true)
        .args(&["disassemble", "simulate"]),
))]
pub(crate) struct Args {
    /// Disassemble the provided file
    #[arg(short = 'd', long = "disassemble")]
    pub disassemble: bool,

    /// Simulate the provided file
    #[arg(short = 's', long = "simulate")]
    pub simulate: bool,

    /// Input file for disassembly or simulation
    #[arg(value_name = "FILE")]
    pub file: String,
}
