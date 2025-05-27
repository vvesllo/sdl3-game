use sdl3::render::Canvas;
use sdl3::video::Window;

pub struct Renderer<'a> {
    canvas: &'a mut Canvas<Window>
}

impl<'a> Renderer<'a> {
    pub fn new(canvas: &'a mut Canvas<Window>) -> Self {
        Self {
            canvas: canvas
        }
    }

    pub fn render(&mut self) {
        self.canvas.clear();

        // render here (i think)

        self.canvas.present();
    }
}