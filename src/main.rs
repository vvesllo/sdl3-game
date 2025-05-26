use sdl3::Sdl;
use sdl3::VideoSubsystem;
use sdl3::EventPump;

use sdl3::video::Window;
use sdl3::render::Canvas;
use sdl3::pixels::Color;
use sdl3::event::Event;

use std::time::Duration;

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

    canvas.set_draw_color(Color::RGB(0x34, 0x40, 0x64));


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

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    };
}
