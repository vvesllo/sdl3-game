use sdl3::{EventPump, Sdl, VideoSubsystem};
use sdl3::video::{Window, WindowContext};
use sdl3::render::{Texture, TextureCreator, Canvas, FPoint, FRect};
use sdl3::keyboard::{KeyboardState, Scancode};
use sdl3::image::LoadTexture;
use sdl3::pixels::Color;
use sdl3::event::Event;
use sdl3::timer;

use std::time::Duration;
use std::thread;

mod player;
use player::Player;

const WINDOW_WIDTH: u32 = 720;
const WINDOW_HEIGHT: u32 = 480;


fn main() -> Result<(), String>{
    let sdl_context: Sdl = sdl3::init().unwrap();
    let video_subsystem: VideoSubsystem = sdl_context
        .video()
        .unwrap();
    let window: Window = video_subsystem
        .window("Hello", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas: Canvas<Window> = window.into_canvas();
    let mut event_pump: EventPump = sdl_context.event_pump().unwrap();

    canvas.set_draw_color(Color::RGB(0x16, 0x16, 0x20));


    let texture_creator: TextureCreator<WindowContext> = canvas
        .texture_creator();
   
    let texture: Texture<'_> = texture_creator
        .load_texture("./horse.jpg")
        .unwrap();
    
    let mut player: Player = Player::new(
        FRect::new(10f32, 10f32, 150f32, 50f32),
        &texture
    );

    let mut last_tick: u64;
    let mut current_tick: u64 = 0;
    
    'running : loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => { break 'running; },
                _ => {}
            }
        }
    
        last_tick = current_tick;
        current_tick = timer::ticks();
        
        let dt: f32 = (current_tick - last_tick) as f32 / 1000.;
    
        canvas.clear();

        let mut velocity: FPoint = FPoint::new(0f32, 0f32);

        let keyboard_state: KeyboardState<'_> = event_pump.keyboard_state();
        
        if keyboard_state.is_scancode_pressed(Scancode::A) {
            velocity.x -= 1f32;
        }
        if keyboard_state.is_scancode_pressed(Scancode::D) {
            velocity.x += 1f32;
        }
        
        if keyboard_state.is_scancode_pressed(Scancode::W) {
            velocity.y -= 1f32;
        }
        if keyboard_state.is_scancode_pressed(Scancode::S) {
            velocity.y += 1f32;
        }
        
        player.set_velocity(velocity);

        player.update(dt);
        
        player.draw(&mut canvas);

        canvas.present();
        thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}