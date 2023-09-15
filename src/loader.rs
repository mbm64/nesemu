use crate::emu::*;
use crate::helper::*;
use std::fs::File;
use std::io::Read;
use std::os::unix::prelude::FileExt;
use crate::ppu::Mirroring;
#[allow(dead_code)]
#[derive(Debug)]
struct Header {
    mapper : u8,
    mirroring: u8,
    prg_ram_present: u8,
    trainer_present: u8,
    four_screen:u8,
    unisystem:u8,
    playchoice:u8,
    nes2:u8,
    prg_rom_size:u8,
    chr_rom_size:u8

}
impl Header {
    fn grab_header(game: &mut File) -> Self{
        let mut header : [u8;16] = [0;16];
        game.read_exact(&mut header[..]).expect("rom to small");
        let mapper = ((header[6] & 0xF0) >> 4) | (header[7] & 0xF0); 
        let mirroring = get_bit(header[6], 0);
        let prg_ram_present = get_bit(header[6], 1);
        let trainer_present = get_bit(header[6], 2);
        let four_screen = get_bit(header[6], 3);
        let unisystem = get_bit(header[7], 0);
        let playchoice = get_bit(header[7], 1);
        let nes2 = (header[7] & 0b1100) >> 2;
        let prg_rom_size = header[4];
        let chr_rom_size = header[5];
        //println!("rom size: {}, chr rom size: {},  mapper: {}, trainer_present:{}", prg_rom_size,chr_rom_size,mapper, trainer_present);
        let head = Header { mapper, mirroring, prg_ram_present, trainer_present, four_screen, unisystem, playchoice, nes2, prg_rom_size, chr_rom_size };
        //println!("{:?}", head);
        head

    }
}
pub struct Rom{
    path:String,
    file: File,
    header: Header

}
impl Rom {
    pub fn open_rom(path: String) -> Self{
        let mut rom = File::open(&path).expect("no file found");
        let header = Header::grab_header(&mut rom);
        Rom { path, file : rom, header}
    }
    pub fn load_rom(&mut self, nes : &mut Nes){
        match self.header.mapper {
            0 => mapper000(nes, self),
            x => panic!("mapper {} not implemented yet", x)
        }

    }
}
pub fn load_game(nes : &mut Nes, game_path: String){
    let mut rom = Rom::open_rom(game_path); 
    
}
pub fn mapper000(nes : &mut Nes, rom: &mut Rom){
    let mut prg_rom_offset = 16;
    if rom.header.trainer_present > 0 {
        prg_rom_offset += 512;
    }
    rom.file.read_at(&mut nes.memory[0x8000..0xBFFF], prg_rom_offset).expect("err reading");
    if rom.header.prg_rom_size > 1 {
        rom.file.read_at(&mut nes.memory[0xC000..0xFFFF], prg_rom_offset + 16384).expect("err reading");
        prg_rom_offset += 0x4000;
    }
    else {
        rom.file.read_at(&mut nes.memory[0xC000..0xFFFF], prg_rom_offset).expect("err reading");
    }
    prg_rom_offset += 0x4000;
    rom.file.read_at(&mut nes.ppu.memory[0..0x2000], prg_rom_offset).expect("err reading rom");
    if rom.header.mirroring == 0 {
        nes.ppu.mirroring = Mirroring::Horizontal;
    }
    else {
        nes.ppu.mirroring = Mirroring::Vertical;
    }
    let bit1 = nes.memory[0xfffc];
    let bit2 = nes.memory[0xfffd];
    nes.pc = endian(bit1, bit2);
    //println!("{:?}",&mut nes.ppu.memory[0..0x2000] );


}
