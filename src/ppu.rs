struct PPU{
    ppuctrl: u8,
    ppumask: u8,
    ppustatus:u8,
    oamaddr:u8,
    ppuscroll:u8,
    ppuaddr:u8,
    ppudata:u8,
    memory: [u8; 0x4000]
}
