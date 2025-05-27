use sdl3::render::Canvas;
use sdl3::{EventPump, Sdl, VideoSubsystem};
use sdl3::event::Event;
use sdl3::video::Window;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;


mod renderer;
use renderer::Renderer;

fn main() -> Result<(), String> {
    let sdl_context: Sdl = sdl3::init()
        .unwrap();
    let video_subsystem: VideoSubsystem = sdl_context
        .video()
        .unwrap();

    let window: Window = video_subsystem
        .window("SDL3", 720, 480)
        .build()
        .unwrap();

    let mut event_pump: EventPump = sdl_context
        .event_pump()
        .unwrap();

    let mut canvas: Canvas<Window> = window
        .into_canvas();
    
    canvas
        .set_draw_color(Color::RGB(0x34, 0x40, 0x64));
    

    let mut renderer: Renderer = Renderer::new(&mut canvas);


    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }

        renderer.render();
    }

    Ok(())
}