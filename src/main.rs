use sdl3::Sdl;
use sdl3::VideoSubsystem;
use sdl3::EventPump;

use sdl3::video::Window;
use sdl3::render::{Canvas, FRect};

use sdl3::pixels::Color;
use sdl3::event::Event;
use sdl3::keyboard::Scancode;
use sdl3::image::LoadTexture;

use std::time::Duration;

mod Vector;
use Vector::Vector2;

fn main()
{
    let sdl_context: Sdl = sdl3::init().unwrap();

    let video_subsystem: VideoSubsystem = sdl_context
        .video()
        .unwrap();

    let window: Window = video_subsystem
        .window("SDL Project", 720, 480)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas: Canvas<Window> = window
        .into_canvas();
    let mut event_pump: EventPump = sdl_context
        .event_pump()
        .unwrap();


    let texture_creator = canvas
        .texture_creator();
   
    let texture = texture_creator
        .load_texture("./horse.jpg")
        .unwrap();
    
    texture.scale_mode();

    canvas.set_draw_color(Color::RGB(0x34, 0x40, 0x64));

    let mut position: Vector2<f32> = Vector2::<f32>::new(0., 0.);

    let mut rect: FRect = FRect::new(0., 0., 128., 128.);

    canvas
        .copy(&texture, None, None)
        .unwrap();
    canvas.present();


    'running: loop {
        canvas.clear();

        for event in event_pump.poll_iter()
        {
            match event
            {
                Event::Quit { .. } => { break 'running; },
                _ => {}
            }
        }
        

        let mut velocity: Vector2<f32> = Vector2::<f32>::new(0., 0.);
        
        let keyboard_state = event_pump
            .keyboard_state();

        
        if keyboard_state.is_scancode_pressed(Scancode::W) {
            velocity.y -= 1.;
        }
        if keyboard_state.is_scancode_pressed(Scancode::S) {
            velocity.y += 1.;
        }
        
        if keyboard_state.is_scancode_pressed(Scancode::A) {
            velocity.x -= 1.;
        }
        if keyboard_state.is_scancode_pressed(Scancode::D) {
            velocity.x += 1.;
        }


        position += velocity;

        rect.set_x(position.x);
        rect.set_y(position.y);

        canvas
            .copy_ex(
                &texture,
                None,
                Some(rect),
                0.,
                None,
                false,
                false,
            ).unwrap();
        canvas.present();


        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    };
}
