mod cpu;

use cpu::control_unit::{self, ControlUnit};
use cpu::instruction_set::Instruction;

fn main() {
    let mut control_unit = ControlUnit::new();


    // Example Program: Add 5 to the accumulator, then AND it with 3
    control_unit.execute(Instruction::Add(5));
    control_unit.execute(Instruction::And(3));


    println!("Accumulator: {}", control_unit.registers.a); // Should Output 1
}
