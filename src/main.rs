mod emu;
mod opcodes;
mod helper;
mod loader;
mod ppu;
mod screen;
mod controller;
use emu::*;
use screen::Screen;
use sdl2::{self, pixels};
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use loader::*;
use std::env::args;
use helper::*;
use std::thread::sleep;
use std::time;

fn main() {  
    
    let mut game = loader::Rom::open_rom(args().nth(1).expect("not valid file"));
    let mut nes = Nes::start();
    let mut nmi = false;
    let mut vblank = false;
    let mut cycles = 7;
    game.load_rom(&mut nes); 
    /*
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

    }*/



    //sdl 
    //
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("nesemu", 1024, 480).position_centered().build().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_logical_size(512, 240).expect("err with logical size");
    let mut event_pump = sdl_context.event_pump().unwrap();
    canvas.set_draw_color(hex_to_color(0));
    canvas.present();
    let mut screen = Screen::new();
    for i in 0..32 {
        for j in 0..30{
                let tile = nes.ppu.get_tile(i + 32*j, 0);
                screen.draw_tile(i*8, j*8, tile);

        }
    }
    screen.update_canvas(&mut canvas);
    canvas.present();
    'gameloop: loop{
        
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape),.. } => break 'gameloop,
                Event::KeyDown {keycode: Some(key), ..} => nes.controller_1.press(key),
                Event::KeyUp { keycode: Some(key), .. } => nes.controller_1.unpress(key),
                _ => ()
            }
        }
        
        if nmi {nes.nmi_interrput()}
        println!("{:0>4X}  op:{:X} {:X} {:X}    A:{:0>2X} X:{:0>2X} Y:{:0>2X} P:{:0>2X} SP:{:0>2X} CYC:{} PPU: Scan: {} CYC{}",nes.pc, nes.memory[nes.pc as usize], nes.memory[nes.pc as usize +1], nes.memory[nes.pc as usize +2], nes.acc, nes.x, nes.y, nes.p, nes.sp, cycles, nes.ppu.scanlines, nes.ppu.cycles);
        let mut cycles_taken = nes.step();
        if nmi { cycles_taken +=7; }
        cycles+= cycles_taken as usize;
        (nmi,vblank) = nes.ppu.tick(cycles_taken as u16*3);
        if vblank {
            let (render_bg, render_sprites) = (nes.ppu.ppumask & 0b1000 > 0, nes.ppu.ppumask & 0b10000 > 0);
            if(render_bg){
            screen.render_background(&mut nes.ppu);}
            if(render_sprites) {
                screen.render_sprites(&mut nes.ppu);
            }
            if (render_bg | render_sprites){
                screen.update_canvas(&mut canvas);
                canvas.present();
            }
            //println!("screen refresh");
            //println!("scanlines : {}, cyc:{}", nes.ppu.scanlines, nes.ppu.cycles);
        }
        
        
        
    }

    
    /*for i in 0..32 {
        for j in 0..30{
                let tile = nes.ppu.get_tile(i + 32*j, 0);
                screen.draw_tile(i*8, j*8, tile);

        }
    }
    screen.update_canvas(&mut canvas);
    canvas.present();
    sleep(time::Duration::from_secs(10));*/

}

