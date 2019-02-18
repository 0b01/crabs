#[allow(pub_use_of_private_extern_crate)]

extern crate quicksilver;
mod prelude;
mod anim;
mod game;
mod crab;
mod sprites;

#[cfg(target_arch="wasm32")]
const MULT: f32 =  5.;
#[cfg(not(target_arch="wasm32"))]
const MULT: f32 =  1.;

use crate::prelude::*;

struct Crabs {
    sprites: Asset<Sprites>,
    game: Game,
    mouse_down: bool,
    pos_x: f32,
    pos_y: f32,
    ctrl: bool,
    shift: bool,
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
            shift: false,
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        if cfg!(target_arch = "wasm32") {
            window.set_size((WIDTH*MULT, HEIGHT*MULT));
        }
        self.sprites.execute(|spr| {
            spr.update_anim(window)?;
            let bg = spr.get_anim_mut("bg").unwrap();
            if bg.played {
                bg.play()?;
            }
            Ok(())
        })?;
        self.game.update(window, &mut self.sprites)?;
        Ok(())
    }


    fn draw(&mut self, window: &mut Window) -> Result<()> {
        self.draw_ui(window)?;
        self.game.draw(window, &mut self.sprites)?;

        // let loc = (self.pos_x, self.pos_y);
        // self.sprites.execute(|spr|{
        //     let anim_frame = spr.get_anim("crab-rest").unwrap().current_frame();
        //     window.draw(
        //         &anim_frame.area().with_center(loc),
        //         Img(anim_frame)
        //     );
        //     Ok(())
        // })?;

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
                    self.game.step(&mut self.sprites);
                    return Ok(());
                }
                self.game.char('\n')
            },
            Event::Key(Key::Escape, ButtonState::Pressed) => { self.game.stop() },
            Event::Key(Key::Space, ButtonState::Pressed) => { self.game.char(' ') },
            Event::Key(Key::Back, ButtonState::Pressed) => { self.game.char('\0') },
            Event::Key(Key::Colon, ButtonState::Pressed) => { self.game.char(':') },
            Event::Key(Key::Semicolon, ButtonState::Pressed) => {
                if self.shift { self.game.char(':') }
            },
            Event::Key(Key::Minus, ButtonState::Pressed) | Event::Key(Key::Subtract, ButtonState::Pressed) => { self.game.char('-') },
            Event::Key(Key::LShift, ButtonState::Pressed) | Event::Key(Key::RShift, ButtonState::Pressed) => {
                self.shift = true;
            },
            Event::Key(Key::LControl, ButtonState::Pressed) | Event::Key(Key::RControl, ButtonState::Pressed) => {
                self.ctrl = true;
            },
            Event::Key(k, ButtonState::Pressed) => { dbg!(k); },

            Event::MouseButton( MouseButton::Left, ButtonState::Pressed) => {
                dbg!(&window.mouse().pos());
                // js!(
                //     // format!("{:#?}", &window.mouse().pos())
                //     console.log(1);
                // );
                let Vector {x, y} = window.mouse().pos();


                macro_rules! click_sound {
                    () => {
                        self.sprites.execute(|i| {
                            i.get_sound("click").unwrap().play()?;
                            Ok(())
                        })?;
                    }
                }

                if x> 21.502869*MULT && x< 30.15857*MULT && y> 8.043931*MULT && y< 18.273455*MULT {
                    click_sound!();
                    // play
                    self.game.play();
                }
                if x > 46.226604*MULT && x < 58.34049*MULT && y > 7.6740127*MULT && y < 19.53377*MULT {
                    click_sound!();
                    self.game.step(&mut self.sprites);
                }
                if x > 74.01056*MULT && x < 83.92191*MULT && y > 7.001367*MULT && y < 18.945835*MULT {
                    click_sound!();
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

            let crab = spr.get_anim("crab-right").unwrap().current_frame();
            window.draw_ex(&
                crab.area().with_center((115.17666, 168.09296)),
                Img(&crab),
                Transform::scale(Vector::new(1, 1)),
                2,
            );

            let crab = spr.get_anim("crab-right").unwrap().current_frame();
            window.draw_ex(&
                crab.area().with_center((138.5878, 132.22208)),
                Img(&crab),
                Transform::scale(Vector::new(1, 1)),
                2,
            );

            Ok(())
        })?;

        Ok(())
    }
}

fn main() {
    run::<Crabs>("Crabs", Vector::new(WIDTH, HEIGHT), Settings {
        resize: quicksilver::graphics::ResizeStrategy::Fit,
        fullscreen: false,
        ..Default::default()
    });
}
