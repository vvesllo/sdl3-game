use sdl3::{EventPump, Sdl, VideoSubsystem};
use sdl3::render::{Canvas, FRect, Texture, TextureCreator};
use sdl3::video::{Window, WindowContext};
use sdl3::event::Event;

use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use sdl3::surface::Surface;

use sdl3::ttf::{Sdl3TtfContext, FontStyle, Font};

use std::path::Path;
use std::thread;
use std::time;


// это пиздец
fn render(canvas: &mut Canvas<Window>, texture_creator: &TextureCreator<WindowContext>, font: &Font) -> Result<(), String> {
    let color: Color = Color::RGB(0x34, 0x40, 0x64);
    canvas.set_draw_color(color);
    canvas.clear();

    let hello_text: String = String::from("Hello, world!");
    let surface: Surface<'_> = font
        .render(&hello_text)
        .blended(Color::RGB(0xA7,0xB1,0xCC))
        .unwrap();
    let texture: Texture<'_>= texture_creator
        .create_texture_from_surface(&surface)
        .unwrap();
    let target: FRect = FRect::new(10., 10., 200., 100.);
    canvas
        .copy(&texture, None, Some(target))
        .unwrap();
        
    canvas.present();
    Ok(())
}


fn main() ->Result<(), String> {
    let sdl_context: Sdl = sdl3::init().unwrap(); 
    let video_subsystem: VideoSubsystem = sdl_context
        .video()
        .unwrap();

    let window: Window = video_subsystem
        .window("SDL3", 720, 480)
        .build()
        .unwrap();

    let mut canvas: Canvas<Window> = window.into_canvas();
    let texture_creator: TextureCreator<WindowContext>= canvas.texture_creator();

    let ttf_context: Sdl3TtfContext = sdl3::ttf::init()
        .unwrap();
    
    let font_path: &Path = Path::new(&"fonts/font.otf");
    let mut font: Font<'_, 'static> = ttf_context
        .load_font(font_path, 128f32).unwrap();
    font.set_style(FontStyle::NORMAL);

    let mut event_pump: EventPump = sdl_context
        .event_pump()
        .unwrap();
 
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

        render(&mut canvas, &texture_creator, &font).unwrap();

        thread::sleep(time::Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}