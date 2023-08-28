use crate::helper::*;
use crate::opcodes::run_op_code;
use std::fs;
pub struct Nes {
    pub memory: [u8; 0x10000], 
    pub pc: u16,
    pub sp: u8,
    pub p:u8,
    pub acc: u8,
    pub x:u8,
    pub y:u8,
    pub page_cross:u8
}
impl Nes{
    pub fn nextOp(&mut self)-> u8{
        let op = self.memory[self.pc as usize];
        self.pc +=1;
        op
    }
    pub fn nextabs(&mut self)-> u16{
        let a1 = self.nextOp() as u16;
        let a2 = self.nextOp() as u16;
        a1 + (a2 << 8)
    }
    pub fn set_flags(&mut self, flags: u8){
        self.p |= flags;
    }
    pub fn reset_flags(&mut self, flags: u8){
        self.p = self.p & (!flags);
    }
    pub fn push_to_stack(&mut self, byte:u8){
        self.memory[ 0x100 + self.sp as usize] = byte;
        //eprintln!("pushing {:#x} to the stack",byte);
        self.sp-=1;
    }
    pub fn pop_from_stack(&mut self) -> u8{
        self.sp +=1;
        let byte = self.memory[0x100 + self.sp as usize];
        //eprintln!("poping {:#x} from the stack", byte);
        byte

    }
    pub fn start() -> Self {
        let emu = Nes{
            memory: [0; 0x10000],
            pc:0xC000,
            sp:0xFD,
            p:0b00100100,
            acc:0,
            x:0,
            y:0,
            page_cross:0
        };

        emu
    }
    pub fn step(&mut self) -> u8 {
        let op = self.nextOp();
        run_op_code(self,op)
    }

    
}
