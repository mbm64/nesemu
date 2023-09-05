use sdl2::pixels::Color;
use sdl2::pixels;
pub fn get_bit(byte: u8, bit: u8)-> u8{

    (byte & (1 << bit)) >> bit
}
pub fn endian(n1:u8,n2:u8) -> u16{
    ((n2 as u16)<< 8) + (n1 as u16)
}
pub fn unendian(c:u16) -> [u8;2]{
    let a = c & 0xFF;
    let b = (c& 0xFF00) >> 8;
    [a as u8, b as u8]
}
pub fn is_negative(a:u8) -> bool {
    if (a as i8) < 0 {
        true
    }
    else {
        false
    }
}
pub fn page_crossed(address:u16, added: u8) -> bool {
   let c = address + added as u16;
   let mask = 0xFF00;
   if (c & mask) != (address & mask){
       return true;
   }
   else{
       return false;
   }
}
pub fn hex_to_color(color:u32) -> Color{
    let r = (color >> 16) & 0xff;
    let g = (color >> 8) & 0xff;
    let b = color & 0xff;
    Color::RGB(r as u8, g as u8, b as u8)
}
