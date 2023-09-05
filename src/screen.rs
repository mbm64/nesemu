use sdl2::render::WindowCanvas;
use sdl2;
use crate::ppu::PPU;
use crate::helper::hex_to_color;
const PALLETE: [u32;0x40] = [
    0x626262, 0x001fb2, 0x2404c8, 0x5200b2, 0x730076, 0x800024, 0x730b00, 0x522800, 0x244400, 0x005700, 0x005c00, 0x005324, 0x003c76, 0x000000, 0x000000, 0x000000,
    0xababab, 0x0d57ff, 0x4b30ff, 0x8a13ff, 0xbc08d6, 0xd21269, 0xc72e00, 0x9d5400, 0x607b00, 0x209800, 0x00a300, 0x009942, 0x007db4, 0x000000, 0x000000, 0x000000,
    0xffffff, 0x53aeff, 0x9085ff, 0xd365ff, 0xff57ff, 0xff5dcf, 0xff7757, 0xfa9e00, 0xbdc700, 0x7ae700, 0x43f611, 0x26ef7e, 0x2cd5e6, 0x4e4e4e, 0x000000, 0x000000,
    0xffffff, 0xb6e1ff, 0xced1ff, 0xe9c3ff, 0xffbcff, 0xffbdf4, 0xffc8c3, 0xffd59a, 0xe9e681, 0xcef481, 0xb6fb9a, 0xa9fac3, 0xa9f0f4, 0xb8b8b8, 0x000000, 0x000000];
pub struct Screen {
    screen: [u32; 256*240],
    pallete: [u8;4]
}
impl Screen {
    pub fn new() -> Self{
        Screen { screen: [0;256*240], pallete: [0x1,0x23,0x27,0x30]}
    }
    pub fn set_pixel(&mut self, x:usize, y:usize, color:u32) {
        let index = (x as usize) + (y as usize)*256;
        self.screen[index] = color;
    }
    pub fn get_pixel(&self, x:usize, y:usize) -> u32{
        self.screen[x + y*256]
    }
    pub fn draw_tile(&mut self, x:usize, y:usize, tile:[u8;64]){
        for i in 0..8 {
            if (x + i) >= 256 { continue; }
            for j in 0..8 {

                if y+j>= 240 {continue;}
                let color = self.get_color(tile[i + 8*j] );
                if color == 0xffffffff {
                    continue;
                }
                self.set_pixel(x+i, y+j, color);
                
            }
        }

    }
    pub fn draw_sprite_behind_bg(&mut self, x:usize, y:usize,bg:u8, tile:[u8;64], overlapped: &mut bool){

        for i in 0..8 {
            if (x + i) >= 256 { continue; }
            for j in 0..8 {

                if y+j>= 240 {continue;}
                let color = self.get_color(tile[i + 8*j] );
                if color == 0xffffffff {
                    continue;
                }
                let bgc = self.get_pixel(x+i, y+j);
                if bgc != PALLETE[bg as usize] {
                    *overlapped = true;
                    continue;
                }
                self.set_pixel(x+i, y+j, color);
                
            }
        }

    }
    pub fn draw_sprite_0(&mut self, x:usize, y:usize,bg:u8, tile:[u8;64],overlapped: &mut bool){
        for i in 0..8 {
            if (x + i) >= 256 { continue; }
            for j in 0..8 {

                if y+j>= 240 {continue;}
                let color = self.get_color(tile[i + 8*j] );
                if color == 0xffffffff {
                    continue;
                    
                }
                let bgc = self.get_pixel(x+i, y+j);
                if bgc != PALLETE[bg as usize] {
                    *overlapped = true;
                    println!("overlap");
                }
                self.set_pixel(x+i, y+j, color);
                
            }
        }
        

    }

    pub fn get_color(& self, index: u8) -> u32 {
        let color_ind = self.pallete[index as usize];
        if color_ind >= 0x40 {
            return 0xffffffff;
        }
        PALLETE[color_ind as usize]

        /*match index {
            0 => PALLETE[0x1],
            1 => PALLETE[0x23],
            2 => PALLETE[0x27],
            3 => PALLETE[0x30],
            _ => panic!("invalid color")
            
        }*/
    }
    pub fn update_canvas(&mut self, canvas:&mut WindowCanvas){
        for i in 0..256 {
            for j in 0..240{
                canvas.set_draw_color(hex_to_color(self.get_pixel(i as usize, j as usize)));
                canvas.draw_point(sdl2::rect::Point::new(i, j)).expect("err drawing point");
            }
        }
    }


    pub fn render_background(&mut self, ppu: &mut PPU){
        let nametable = ppu.ppuctrl & 0b11;
        let starting_address = match nametable {
            0 => 0x2000,
            1 => 0x2400,
            2 => 0x2800,
            3 => 0x2c00, 
            _ => panic!("render error selecting nametable")
        };
        let bank = (ppu.ppuctrl & 0b10000) >> 4; 
        for i in 0..0x3c0 {
           let tilenum = ppu.read_ppudata_add(starting_address + i); 
           let pallete_num = i/4;
           let tile = ppu.get_tile(tilenum as usize, bank as usize);
           let x = i % 32;
           //let pal_x = i / 4 % 16;
           let pal_x = x /4;
           let y = i/32;
           //let pal_y = i / 0x80;
           let pal_y = y / 4;
           let pallete = ppu.read_ppudata_add(starting_address + 0x3c0 + pal_x + pal_y*8);
           let pal = match (x % 4 / 2, y % 4 / 2) {
               (0,0) => pallete & 0b11,
               (1,0) => (pallete >> 2) & 0b11,
               (0,1) => (pallete >> 4) & 0b11,
               (1,1) => (pallete >> 6) & 0b11,

               _ => panic!("error with pallete")

           };
           let pallete_data = ppu.get_pallete(pal);
           if pal == 2 {
               //println!("pallet 2 with the blue {:?}", pallete_data);

           }

           self.pallete = pallete_data;

           self.draw_tile((x*8) as usize, (y*8) as usize, tile);
        }

    }
    
    pub fn render_sprites(&mut self, ppu: &mut PPU){
        for i in (0..256).step_by(4).rev() {
            let x = ppu.oam[i+3];
            let y = ppu.oam[i];
            let bank = (ppu.ppuctrl & 0b1000) >>3;
            let tile_num = ppu.oam[i+1];
            let byte2 = ppu.oam[i+2];
            let pallete_num = byte2 & 0b11;
            let flip_hor = (byte2 & 0b1000000) >> 6;
            let flip_ver = byte2 >> 7;
            let prio = (byte2 & 0b100000);
            let tile = ppu.get_tile(tile_num as usize, bank as usize);
            self.pallete = ppu.get_sprite_pallete(pallete_num);
            let flipped_tile = match (flip_hor, flip_ver) {
                (0,0) => tile,
                (1,0) => flip_tile_horizontal(tile),
                (0,1) => flip_tile_vertical(tile),
                (1,1) => flip_both(tile),
                _ => panic!("error in tile render")
                
            };
            let mut overlap = false;
            if prio > 0{
                let bg = ppu.read_ppudata_add(0x3f00);
                self.draw_sprite_behind_bg(x as usize, y as usize, bg, flipped_tile,&mut overlap);
                
            }
            else{
                let bg = ppu.read_ppudata_add(0x3f00);
                self.draw_sprite_0(x as usize, y as usize, bg, flipped_tile,&mut overlap);

            }
            /*else {
                self.draw_tile(x as usize,y as usize, flipped_tile);
            }*/
            if overlap  && i == 0{
                //println!("overlap");
                    ppu.ppustatus |= 0b01000000;
            }


        }
    }
    
    
}
pub fn flip_tile_vertical(tile: [u8;64]) -> [u8;64] {
    let mut flipped = [0;64];
    for x in 0.. 8 {
        for y in 0..8 {
            flipped[x+ (7-y)*8] = tile[x + y*8];

        }

    }
    flipped

}
pub fn flip_tile_horizontal(tile: [u8;64]) -> [u8;64] {

   let mut flipped = [0;64];
    for x in 0.. 8 {
        for y in 0..8 {
            flipped[ 7-x + (y)*8] = tile[x + y*8];

        }

    }
    flipped

}
pub fn flip_both(tile: [u8;64]) -> [u8;64] {
   let mut flipped = [0;64];
    for x in 0.. 8 {
        for y in 0..8 {
            flipped[ 7-x + (7-y)*8] = tile[x + y*8];

        }

    }
    flipped


}
