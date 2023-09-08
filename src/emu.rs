use crate::helper::*;
use crate::opcodes::run_op_code;
use std::fs;
use crate::ppu::*;
use crate::controller::Controller;
pub struct Nes {
    pub memory: [u8; 0x10000], 
    pub pc: u16,
    pub sp: u8,
    pub p:u8,
    pub acc: u8,
    pub x:u8,
    pub y:u8,
    pub page_cross:u8,
    pub ppu:PPU,
    pub controller_1: Controller

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
    pub fn grababs(&mut self) -> u16{
        let abs = self.nextabs();
        self.pc -= 2;
        abs

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
            pc:0x8000,
            sp:0xFD,
            p:0b00100100,
            acc:0,
            x:0,
            y:0,
            page_cross:0,
            ppu: PPU::new(),
            controller_1: Controller::new()
        };

        emu
    }
    pub fn step(&mut self) -> u8 {
        let op = self.nextOp();
        run_op_code(self,op)
    }
    pub fn read_memory(&mut self, address: u16) -> u8 {
        match address {
            0x0..=0x7FF => self.memory[address as usize] ,
            0x800..=0x1FFF => self.read_memory(address % 0x800),
            0x2002 => self.ppu.read_ppustatus(),
            0x2004 => self.ppu.read_oamdata(),
            0x2007 => self.ppu.read_ppudata(),
            0x2008..=0x3FFF => self.read_memory((address-0x2000)%8 + 0x2000),
            0x4016 => self.controller_1.read(),
            0x4000..=0x401F => 0,
            0x4020..=0xFFFF => self.memory[address as usize],
            _ => panic!("invalid read into cpu memory at {}", address)
        }

    }
    pub fn write_memory(&mut self, address: u16, value:u8){
        match address {
            0x0..=0x7FF => {self.memory[address as usize] = value;},
            0x800..= 0x1FFF => {
                self.write_memory(address % 0x800, value)
            },
            0x2000 => self.ppu.write_ppuctrl(value),
            0x2001 => self.ppu.write_ppumask(value),
            0x2003 => self.ppu.write_oamaddr(value),
            0x2004 => self.ppu.write_oamdata(value),
            0x2005 => self.ppu.write_ppuscroll(value),
            0x2006 => self.ppu.write_ppuaddr(value),
            0x2007 => self.ppu.write_ppudata(value),
            0x4014 => {
                let add = (value as u16) << 8;
                let mut slice = [0;256];
                let oamshift = self.ppu.oamaddr;
                for i in 0..0xff {
                    slice[ (i+ oamshift)  as usize] = self.read_memory(add + i as u16 );
                }
                self.ppu.write_oamdma(slice, value)
            }
            ,
            0x2008..= 0x3fff => self.write_memory((address - 0x2000) % 8 + 0x2000, value),
            0x4016 => self.controller_1.write(value),
            //to do apu io
            0x4000..= 0x401f => (),


            _ => panic!("writing to invalid address {}", address)
        };
    }
    pub fn nmi_interrput(&mut self){
        let bytes = unendian(self.pc);
        self.push_to_stack(bytes[1]);
        self.push_to_stack(bytes[0]);
        self.push_to_stack(self.p);
        self.p |= 0b100;
        let byte1 = self.read_memory(0xFFFA);
        let byte2 = self.read_memory(0xfffB);
        self.pc = endian(byte1, byte2);
        //println!("nmi interupt");

    }


    
}
