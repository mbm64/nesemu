use std::result;


use crate::emu::Nes;
use crate::{helper::*, screen::*};
pub enum Mirroring {
    Horizontal, Vertical, SingleScreen, FourScreen, NotSet
}
struct AddressRegister{
    hi_byte:u8,
    lo_byte:u8,
    first_byte:bool
}
struct ScrollRegister{
    x:u8,
    y:u8,
    setting_x:bool
}
impl ScrollRegister{
    pub fn set(&mut self, byte:u8){
        if self.setting_x {
            self.x = byte;
        }
        else {
            self.y = byte;
            //println!("{},{}",self.x,self.y);
        }
        
        self.setting_x = !self.setting_x;
    }
    fn new () -> Self {
        ScrollRegister{
            x:0,
            y:0,
            setting_x:true
        }
    }
}
impl AddressRegister {
    fn address(&self) -> u16{
        (self.lo_byte as u16) + ((self.hi_byte as u16) << 8) 
    }
    fn write(&mut self, value :u8){
        if self.first_byte {
            self.hi_byte = value;
        }
        else {
            self.lo_byte = value;
            println!("byte set to {:#X}", self.address());
        }
        if self.address() >= 0x4000 {
            println!("mirrored the address of {:#x}", self.address());

            self.hi_byte &= 0b111111;
           println!("mirrored down to {:#x}", self.address());
        }
        self.first_byte = !self.first_byte;

    }
    fn add(&mut self, value :u8){
        let new_value = self.address() + value as u16;
        self.lo_byte = (new_value & 0xFF) as u8;
        self.hi_byte = (new_value >> 8) as u8;
    }
    fn new() -> Self{
        AddressRegister { hi_byte: 0, lo_byte: 0, first_byte: true }
    }
    
}
pub struct PPU{
    pub ppuctrl: u8,
    pub ppumask: u8,
    pub ppustatus:u8,
    oamaddr:u8,
    oamdata:u8,
    ppuscroll:u8,
    ppuaddr:u8,
    ppudata:u8,
    oamdma:u8,
    pub oam:[u8;256],
    pub memory: [u8; 0x4000],
    address_register: AddressRegister,
    internal_buffer: u8,
    pub mirroring: Mirroring, 
    scroll_register: ScrollRegister,
    pub scanlines:u16,
    pub cycles:u16
}
impl PPU{
    pub fn new() -> Self{
        PPU{
            ppuctrl: 0,
            ppumask: 0,
            ppustatus:0,
            oamaddr:0,
            oamdata:0,
            ppuscroll:0,
            ppuaddr:0,
            ppudata:0,
            oamdma:0,
            oam:[0;256],
            memory: [0; 0x4000],
            address_register: AddressRegister::new(),
            internal_buffer: 0,
            mirroring: Mirroring::NotSet, 
            scroll_register: ScrollRegister::new(), 
            scanlines:0,
            cycles:0

        }
    }
    /*pub fn get_registers(&mut self, nes: &Nes){
        self.ppuctrl = nes.read_memory(0x2000);
        self.ppumask = nes.read_memory(0x2001);
        self.ppustatus = nes.read_memory(0x2002);
        self.oamaddr = nes.read_memory(0x2003);
        self.oamdata = nes.read_memory(0x2004);
        self.ppuscroll = nes.read_memory(0x2005);
        self.ppuaddr = nes.read_memory(0x2006);
        self.ppudata = nes.read_memory(0x2007);
        self.oamdma = nes.read_memory(0x4014);
        
    }
    pub fn set_registers(&self, nes: &mut Nes){
        nes.write_memory(0x2000, self.ppuctrl);
        nes.write_memory(0x2001, self.ppumask);
        nes.write_memory(0x2002, self.ppustatus);
        nes.write_memory(0x2003, self.oamaddr);
        nes.write_memory(0x2004, self.oamdata);
        nes.write_memory(0x2005, self.ppuscroll);
        nes.write_memory(0x2006, self.ppuaddr);
        nes.write_memory(0x2007, self.ppudata);
        nes.write_memory(0x4014, self.oamdma);
    }*/ 

    pub fn write_ppuaddr(&mut self, byte:u8){
        //println!("write to address");
        self.address_register.write(byte);

    }
    pub fn write_ppuctrl(&mut self, byte:u8){
        self.ppuctrl = byte;
    }
    pub fn write_ppumask(&mut self, byte:u8){
        self.ppumask = byte;
    }
    pub fn read_ppustatus(&mut self) -> u8{
        let temp = self.ppustatus;
        self.ppustatus &= !0b10000000;
        self.address_register.first_byte = true;
        temp
    }
    pub fn write_oamaddr(&mut self,byte:u8){
        //println!("oamaddr write of {}",byte);
        self.oamaddr = byte;
        self.oamdata = self.oam[self.oamaddr as usize];

    }
    pub fn read_oamdata(&mut self) -> u8{
        //println!("oamdata read");
        self.oamdata

    }
    pub fn write_oamdata(&mut self, byte:u8){
        //println!("oamdata write");
        self.oam[self.oamaddr as usize] = byte;
        self.write_oamaddr(self.oamaddr+1);
    }
    pub fn write_ppuscroll(&mut self,byte:u8){
        //println!("write to scroll");
        self.scroll_register.set(byte);
    }
    pub fn read_ppudata(&mut self) -> u8{
        let address = self.address_register.address();
        self.increment_ppuaddr();
        let data = self.internal_buffer;
        match address {
            0..=0x1fff => {
                self.internal_buffer = self.memory[address as usize];
            },
            0x2000..=0x2fff => {
                self.internal_buffer = self.memory[self.vram_address(address)];
            },
            0x3f00..=0x3fff => {
                self.internal_buffer = self.memory[pallete_mirror(address)];
                return self.internal_buffer;
            },
            0x3000..=0x3eff => {
                let mirror = address - 0x1000;
                self.internal_buffer = self.memory[self.vram_address(mirror)];
            },
            _ => {
                panic!("unexpected address {:#x} when trying to access ppu memory", address);
            }
            
        };
        data
        

    }
    pub fn read_ppudata_add(&mut self, address: u16) -> u8{
        match address {
            0..=0x1fff => {
                self.memory[address as usize]
            },
            0x2000..=0x2fff => {
                self.memory[self.vram_address(address)]
            },
            0x3f00..=0x3fff => {
                self.memory[pallete_mirror(address)]
            },
            0x3000..=0x3eff => {
                let mirror = address - 0x1000;
                self.memory[self.vram_address(mirror)]
            },

            _ => {
                panic!("unexpected address {:#x} when trying to access ppu memory", address);
            }
            
        }
        

    }
    pub fn write_ppudata(&mut self, data:u8){
        
        let address = self.address_register.address();
       match address {
            0..=0x1fff => {
                self.memory[address as usize] = data;
            },
            0x2000..=0x2fff => {
                self.memory[self.vram_address(address)] = data;
            },
            0x3f00..=0x3fff => {
                
                
                self.memory[pallete_mirror(address)] = data;
            },
            0x3000..=0x3eff => {
                let mirror = address - 0x1000;
                self.memory[self.vram_address(mirror)] = data;
            },

            _ => {
                panic!("unexpected address {:#x} when trying to access ppu memory", address);
            }
            
        };
       self.increment_ppuaddr();
    }
    pub fn write_oamdma(&mut self, memslice: [u8;256], byte:u8){
        self.oamdma = byte;
        self.oam = memslice;
        //println!("oamdma slice write");
        /*
        let shifted = (byte as u16) << 8;
        for i in 0..= 0xff {
            self.oam[i as usize] = nes.read_memory(shifted + i); 
        }*/

        //println!("todo implement stall of the cpu");
    }
    fn vram_address(&mut self, address: u16) -> usize {
        let relative_address = address - 0x2000;
        let relative_shift = match self.mirroring {
            Mirroring::FourScreen => relative_address,
            Mirroring::SingleScreen => relative_address % 0x400,
            Mirroring::Vertical => relative_address% 0x800,
            Mirroring::Horizontal => relative_address & !0b10000000000,
            Mirroring::NotSet => panic!("mirroring has not been set by mapper")
            
        };
        (0x2000 + relative_shift) as usize
    }
    fn increment_ppuaddr(&mut self){
        if self.ppuctrl & 0b100 > 0 {
            self.address_register.add(32);
        }
        else{
            self.address_register.add(1);
        }
    }
    

    pub fn tick(&mut self, cycles: u8) -> (bool,bool){
       self.cycles += cycles as u16;
       let mut nmi = false;
       let mut vblank = false;
       if self.cycles >= 341 {
           self.cycles -=341;
           self.scanlines+=1;

           if self.scanlines == 241 {
               //todo!("vblank");
               self.ppustatus |= 0b10000000;
               vblank = true;
               //println!("{:#b}", self.ppuctrl);
               if self.ppuctrl & 0b10000000 > 0 {
                   
               nmi = true;
               }
           }
           if self.scanlines == 262 {
               self.scanlines = 0;
               self.ppustatus &= !0b11000000;
               //todo!("last scanline");
           }
       }
       (nmi,vblank)

    }
    pub fn get_tile(&self, tile: usize, bank:usize) -> [u8;64]{
        let tile_sprite_1 = &self.memory[(bank*0x1000 + tile*16) .. (bank* 0x1000 + tile*16 + 8)];
        let tile_sprite_2 = &self.memory[(bank*0x1000 + tile*16 + 8) .. (bank* 0x1000 + tile*16 + 16)];
        let mut tile_sprite = [0;64];
        for i in 0..8 {
            for j in (0..8).rev(){
                let bit0 = get_bit(tile_sprite_1[i as usize], j);
                let bit1 = get_bit(tile_sprite_2[i as usize], j);
                tile_sprite[((7-j)+ i*8) as usize] = (bit1 << 1) | (bit0);
            }
        }
        tile_sprite

    }
    pub fn get_pallete(& self, pallete_num: u8) -> [u8;4]{
        let mut pal = [0;4];
        pal[0] = self.memory[0x3f00];
        let pal_ind = 0x3f01 + pallete_num as usize*4;
        pal[1] = self.memory[pal_ind];
        pal[2] = self.memory[pal_ind+1];
        pal[3] = self.memory[pal_ind+2];

        pal

    }
    pub fn get_sprite_pallete(& self, pallete_num: u8) -> [u8;4]{
        let mut pal = [0;4];
        pal[0] = 0xff;
        let pal_ind = 0x3f11 + pallete_num as usize*4;
        pal[1] = self.memory[pal_ind];
        pal[2] = self.memory[pal_ind+1];
        pal[3] = self.memory[pal_ind+2];

        pal

    }
    

}
pub fn pallete_mirror(address : u16) -> usize {
    let mirrored_address = ((address - 0x3f20) % 0x20) + 0x3f00;
    let mirror = match mirrored_address {
        0x3f10 => 0x3f00,
        0x3f14 => 0x3f04,
        0x3f18 => 0x3f08,
        0x3f1c => 0x3f1c, 
        _ => mirrored_address
    };
    mirror as usize
}
