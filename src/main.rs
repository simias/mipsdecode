use std::fmt::{Display, Formatter, Error};

mod asm;

fn main() {

    let args: Vec<_> = std::env::args().skip(1).collect();

    if args.is_empty() {
        panic!("Usage: mipsdecode instruction [instruction [...]]");
    }

    for arg in args {
        match u32_from_str_prefix(&arg) {
            None    => println!("Not a valid instruction: {}", arg),
            Some(i) => println!("0x{:08x} -> {}",
                                i,
                                asm::decode(Instruction(i))),
        }
    }
}

#[derive(Clone,Copy)]
pub struct Instruction(u32);

impl Instruction {
    /// Return bits [31:26] of the instruction
    fn function(self) -> u32 {
        let Instruction(op) = self;

        op >> 26
    }

    /// Return bits [5:0] of the instruction
    fn subfunction(self) -> u32 {
        let Instruction(op) = self;

        op & 0x3f
    }

    /// Return coprocessor opcode in bits [25:21]
    fn cop_opcode(self) -> u32 {
        let Instruction(op) = self;

        (op >> 21) & 0x1f
    }

    /// Return register index in bits [25:21]
    fn s(self) -> RegisterIndex {
        let Instruction(op) = self;

        RegisterIndex((op >> 21) & 0x1f)
    }

    /// Return register index in bits [20:16]
    fn t(self) -> RegisterIndex {
        let Instruction(op) = self;

        RegisterIndex((op >> 16) & 0x1f)
    }

    /// Return register index in bits [15:11]
    fn d(self) -> RegisterIndex {
        let Instruction(op) = self;

        RegisterIndex((op >> 11) & 0x1f)
    }

    /// Return immediate value in bits [16:0]
    fn imm(self) -> u32 {
        let Instruction(op) = self;

        op & 0xffff
    }

    /// Return immediate value in bits [16:0] as a sign-extended 32bit
    /// value
    fn imm_se(self) -> u32 {
        let Instruction(op) = self;

        let v = (op & 0xffff) as i16;

        v as u32
    }

    /// Shift Immediate values are stored in bits [10:6]
    fn shift(self) -> u32 {
        let Instruction(op) = self;

        (op >> 6) & 0x1f
    }

    /// Jump target stored in bits [25:0]
    fn imm_jump(self) -> u32 {
        let Instruction(op) = self;

        op & 0x3ffffff
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        try!(write!(f, "{:08x}", self.0));

        Ok(())
    }
}

#[derive(Clone,Copy)]
struct RegisterIndex(u32);

fn u32_from_str_prefix(string: &str) -> Option<u32> {
    let (base, num) = match string.len() {
        0 | 1 => (10, string),
        _     =>
            if string.chars().nth(0).unwrap() == '0' {
                match string.chars().nth(1).unwrap() {
                    'x' => (16, &string[2..]),
                    'o' => (8,  &string[2..]),
                    'b' => (2,  &string[2..]),
                     _  => (10, string),
                }
            } else {
                (10, string)
            },
        };

    u32::from_str_radix(num, base).ok()
}
