extern crate tokio_core;
extern crate piston_window;

use piston_window::*;

mod game;
use game::*;

impl Snake {
    fn draw<G>(&self, transform: math::Matrix2d, g: &mut G) where G: Graphics {
        for rect in self.to_body_rectangles() {
            rectangle(self.color, rect, transform, g);
        }
    }

    fn to_body_rectangles(&self) -> Vec<[f64; 4]> {
        self.to_body_positions()
            .iter()
            .map(|pos| Snake::body_rect(pos, 25.0))
            .collect::<Vec<[f64; 4]>>()
    }

    fn body_rect(pos: &Pos, size: f64) -> [f64; 4]  {
        [(pos.x as f64) * size, (pos.y as f64) * size, size, size]
    }
}

impl GameState {
    fn render<G>(&self, args: &RenderArgs, c: Context, g: &mut G) where G: Graphics {
        clear([0.0; 4], g);

        for snake in &self.snakes {
            snake.draw(c.transform, g);
        }
    }

    fn key_press(&mut self, key: Key) {
        match key {
            Key::Up => self.snakes[0].go(Dir::Up),
            Key::Down => self.snakes[0].go(Dir::Down),
            Key::Left => self.snakes[0].go(Dir::Left),
            Key::Right => self.snakes[0].go(Dir::Right),
            _ => ()
        }
    }
}

fn main() {
    // Start communication with game server
    // TODO

    // Graphics loop

    let mut window: PistonWindow =
        WindowSettings::new("Culebra!", [1000, 1000])
            .exit_on_esc(true).build().unwrap();

    let mut gs = GameState::new();

    let mut events = window.events();

    while let Some(e) = events.next(&mut window) {
        if let Some(ref args) = e.render_args() {
            window.draw_2d(&e, |c, g| {
                gs.render(args, c, g);
            });
        }
        if let Some(Button::Keyboard(key)) = e.press_args() {
            gs.key_press(key);
        }
    }
}
