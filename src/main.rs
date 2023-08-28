mod emu;
mod opcodes;
mod helper;
mod loader;
mod ppu;
use emu::*;

use loader::*;
use std::env::args;

fn main() {    
    let num : u8 = 0b10000000;
    let num2 = num as u32;
    let mut game = loader::Rom::open_rom(args().nth(1).expect("not valid file"));
    let mut nes = Nes::start();
    game.load_rom(&mut nes); 
    //load_game(args().nth(1).expect("no rom here"));
    let mut cycles = 7;
    let mut temp = 0;
    let mut instructions = 0;
    loop{
        if temp == 0 {
            instructions +=1;
            //println!("{:#x}  op:{:#x}     A:{:#x} X:{:#x} Y:{:#x} P:{:#x} SP:{:#x} CYC:{}",nes.pc, nes.memory[nes.pc as usize], nes.acc, nes.x, nes.y, nes.p, nes.sp, cycles);
            temp = nes.step();
            
        }
        temp -=1;
        cycles+=1;
        if instructions > 8990 {
            break;
        }

    }

    

}
