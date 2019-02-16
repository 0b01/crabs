extern crate quicksilver;
mod prelude;
mod anim;
mod game;
mod crab;
mod sprites;

use crate::prelude::*;

struct Crabs {
    sprites: Asset<Sprites>,
    game: Game,
    mouse_down: bool,
    pos_x: f32,
    pos_y: f32,
    ctrl: bool,
}

impl State for Crabs {

    fn new() -> Result<Crabs> {
        let sprites = Asset::new(Sprites::new());
        let game = Game::new();
        Ok(Crabs {
            sprites,
            game,
            mouse_down: false,
            pos_x: 0.,
            pos_y: 0.,
            ctrl: false,
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        self.sprites.execute(|spr| {
            spr.update_anim(window)?;
            let bg = spr.get_anim_mut("bg").unwrap();
            if bg.played {
                bg.play()?;
            }
            Ok(())
        })?;
        self.game.update(window)?;
        Ok(())
    }


    fn draw(&mut self, window: &mut Window) -> Result<()> {
        self.draw_ui(window)?;
        self.game.draw(window, &mut self.sprites)?;

        let loc = (self.pos_x, self.pos_y);

        // TODO: remove me
        self.sprites.execute(|spr|{
            let crab = spr.get_img("crab").unwrap();
            window.draw(
                &crab.area().with_center(loc),
                Img(&crab)
            );
            Ok(())
        })?;

        Ok(())
    }

    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        match event {
            Event::Key(Key::Left, ButtonState::Pressed) => {
                self.game.cursor_left();
            }
            Event::Key(Key::Right, ButtonState::Pressed) => {
                self.game.cursor_right();
            }


            Event::Typed(c) => { self.game.char(char::to_ascii_uppercase(c)); }
            Event::Key(Key::C, ButtonState::Pressed) => {
                if self.ctrl {
                    self.game.stop();
                    return Ok(());
                }
            }
            Event::Key(Key::Return, ButtonState::Pressed) => {
                if self.ctrl {
                    self.game.step();
                    return Ok(());
                }
                self.game.char('\n')
            },
            Event::Key(Key::Space, ButtonState::Pressed) => { self.game.char(' ') },
            Event::Key(Key::Back, ButtonState::Pressed) => { self.game.char('\0') },
            Event::Key(Key::Colon, ButtonState::Pressed) => { self.game.char(':') },
            Event::Key(Key::Subtract, ButtonState::Pressed) => { self.game.char('-') },
            Event::Key(Key::LControl, ButtonState::Pressed) | Event::Key(Key::RControl, ButtonState::Pressed) => {
                self.ctrl = true;
            },
            Event::Key(k, ButtonState::Pressed) => { dbg!(k); },

            Event::MouseButton( MouseButton::Left, ButtonState::Pressed) => {
                dbg!(&window.mouse().pos());
                let Vector {x, y} = window.mouse().pos();



                if x> 21.502869 && x< 30.15857 && y> 8.043931 && y< 18.273455 {
                    // play
                    self.game.play();
                }
                if x > 46.226604 && x < 58.34049 && y > 7.6740127 && y < 19.53377 {
                    self.game.step();
                }
                if x > 74.01056 && x < 83.92191 && y > 7.001367 && y < 18.945835 {
                    self.game.stop();
                }

                self.mouse_down = true;
            }
            Event::Key(Key::LControl, ButtonState::Released) | Event::Key(Key::RControl, ButtonState::Released) => {
                self.ctrl = false;
            }
            Event::MouseButton( MouseButton::Left, ButtonState::Released) => {
                self.mouse_down = false;
            }

            Event::MouseMoved(v) => {
                if self.mouse_down {
                    self.pos_x = v.x;
                    self.pos_y = v.y;
                }
            }
            _ => { }
        };
        Ok(())
    }

}

impl Crabs {
    fn draw_ui(&mut self, window: &mut Window) -> Result<()> {
        self.sprites.execute(|spr| {
            let anim = spr.get_anim("bg").unwrap();
            anim.draw(window, 0., 0., 1.);
            Ok(())
        })?;

        Ok(())
    }
}

fn main() {
    run::<Crabs>("Crabs", Vector::new(WIDTH, HEIGHT), Settings::default());
}