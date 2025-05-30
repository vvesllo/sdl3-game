use sdl3::video::Window;
use sdl3::render::{Canvas, FPoint, FRect, Texture};



fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + t * (b - a)
}

fn lerp_point(a: FPoint, b: FPoint, t: f32) -> FPoint {
    FPoint::new(
        lerp(a.x, b.x, t),
        lerp(a.y, b.y, t)
    )
}


pub struct Player<'a> {
    texture: &'a Texture<'a>,
    speed: f32,
    h_flip: bool,
    v_flip: bool,
    current_velocity: FPoint,
    terminal_velocity: FPoint,
    rect: FRect,
    center: FPoint,
}

impl<'a> Player<'a> {
    pub fn new(rect: FRect, texture: &'a Texture<'a>) -> Self {
        Self {
            texture: texture,
            speed: 100f32, 
            h_flip: false,
            v_flip: false,
            current_velocity: FPoint::new(0f32, 0f32),
            terminal_velocity: FPoint::new(0f32, 0f32),
            rect: rect,
            center: FPoint::new(
                rect.w / 2f32, 
                rect.h / 2f32
            )
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>) {
        canvas.copy_ex(
            self.texture,
            None,
            Some(self.rect),
            0f64,
            Some(self.center),
            self.h_flip,
            self.v_flip,
        ).unwrap();
    }

    pub fn set_velocity(&mut self, velocity: FPoint) {
        self.terminal_velocity = velocity;
    }

    pub fn update(&mut self, dt: f32) {
        self.current_velocity = lerp_point(
            self.current_velocity,
            self.terminal_velocity,
            10f32 * dt
        );

        if self.current_velocity.x > 0f32 { self.h_flip=false; }
        if self.current_velocity.x < 0f32 { self.h_flip=true; }

        self.rect.set_xy(
            FPoint::new(
                self.rect.x + self.speed * self.current_velocity.x * dt,
                self.rect.y + self.speed * self.current_velocity.y * dt
            )
        );
    }
}