extern crate piston_window;

use piston_window::*;

struct Game {
    rotation: f64,
    x: f64,
    y: f64,
    up_d: bool, down_d: bool, left_d: bool, right_d: bool
}

impl Game {  
    fn new() -> Game {
        Game { rotation : 0.0, x : 0.0, y : 0.0, up_d: false, down_d: false, left_d: false, right_d: false }
    }

    fn on_update(&mut self, upd: UpdateArgs) {
        self.rotation += 3.0 * upd.dt;
        if self.up_d {
            self.y += (-50.0) * upd.dt;
        }
        if self.down_d {
            self.y += (50.0) * upd.dt;
        }
        if self.left_d {
            self.x += (-50.0) * upd.dt;
        }
        if self.right_d {
            self.x += (50.0) * upd.dt;
        }
    }

    fn on_draw(&mut self, e: Input, ren: RenderArgs, w: &mut PistonWindow) {
        w.draw_2d(&e, |c, g| {
            clear([0.0, 0.0, 0.0, 1.0], g);
            let center = c.transform.trans((ren.width / 2) as f64, (ren.height / 2) as f64);
            let square = rectangle::square(0.0, 0.0, 100.0);
            let red = [1.0, 0.0, 0.0, 1.0];
            rectangle(red, square, center.trans(self.x, self.y).rot_rad(self.rotation).trans(-50.0, -50.0), g); 
        });
    }

    fn on_press(&mut self, inp: Input) {
        match inp {
            Input::Press(but) => {
                match but {
                    Button::Keyboard(Key::Up) => {
                        self.up_d = true;
                    }
                    Button::Keyboard(Key::Down) => {
                        self.down_d = true;
                    }
                    Button::Keyboard(Key::Left) => {
                        self.left_d = true;
                    }
                    Button::Keyboard(Key::Right) => {
                        self.right_d = true;
                    }
                    _ => {}
                }
            }
           _ => {}
        }
    }

    fn on_release(&mut self, inp: Input) {
        match inp {
            Input::Release(but) => {
                match but {
                    Button::Keyboard(Key::Up) => {
                        self.up_d = false;
                    }
                    Button::Keyboard(Key::Down) => {
                        self.down_d = false;
                    }
                    Button::Keyboard(Key::Left) => {
                        self.left_d = false;
                    }
                    Button::Keyboard(Key::Right) => {
                        self.right_d = false;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
 }

fn main() {
    let mut window: PistonWindow = WindowSettings::new(
        "piston-tutorial",
        [600, 600]
    )
    .exit_on_esc(true)
    .build()
    .unwrap();

    let mut game = Game::new();
    while let Some(e) = window.next() {
        match e {
            Input::Render(r) => {
                game.on_draw(e, r, &mut window);
            }
            Input::Update(u) => {
                game.on_update(u);
            }
            Input::Press(_) => {
                game.on_press(e);           
            }
            Input::Release(_) => {
                game.on_release(e);
            }
            _ => {} 
        }
    }
}
