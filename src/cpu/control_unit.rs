use crate::cpu::registers::Registers;
use crate::cpu::alu::ALU;
use crate::cpu::memory::Memory;
use crate::cpu::instruction_set::Instruction;


pub struct ControlUnit {
    pub registers: Registers,
    pub memory: Memory,
}


impl ControlUnit {
    pub fn new() -> Self {
        ControlUnit {
            registers: Registers::new(),
            memory: Memory::new(),
        }
    }

    pub fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Add(val) => {
                self.registers.a = ALU::add(self.registers.a, val);
            }
            Instruction::And(val) => {
                self.registers.a = ALU::and(self.registers.a, val);
            }
        }
    }
}
