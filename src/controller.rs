use crate::helper::*;
use sdl2::keyboard::Keycode;
pub struct Controller {
    reset: bool, 
    register : u8,
    index: u8
}
impl Controller {
    pub fn new() -> Self{
        Controller { reset: false, register: 0, index: 0 }
    }
    pub fn write (&mut self, data: u8){
        self.reset = data & 1 == 1;
        if self.reset {
            self.index  = 0;
        }
    }
    pub fn read(&mut self) -> u8{
        if self.index > 7 {
        return 1;
        }
        let bit = get_bit(self.register,self.index);
        //println!("read bit {}, from register {:#b}", bit, self.register);
        if !self.reset && self.index < 8 {
            self.index+=1;
        }
        bit
    }
    pub fn press(&mut self, button:Keycode){
        let key = keycode_to_bit(button);
        if key > 7 { return (); }
        self.register |= 1 << key;
        //println!("pressed key {:?}, register is now {}", button, self.register);

    }
    pub fn unpress(&mut self, button:Keycode){
        let key = keycode_to_bit(button);
        if key > 7 { return (); }
        self.register &= !(1<<key);
    }


    
}
fn keycode_to_bit(key:Keycode) -> u8{
    match key {
        Keycode::O => 0,
        Keycode::P => 1,
        Keycode::I => 2,
        Keycode::U => 3,
        Keycode::W => 4,
        Keycode::S => 5,
        Keycode::A => 6,
        Keycode::D => 7,

        _ => 0b11111111

    }
}
