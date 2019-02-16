pub const WIDTH: f32 = 480.;
pub const HEIGHT: f32 = 270.;


pub use quicksilver::{
    geom::{Circle, Line, Rectangle, Transform, Triangle, Vector},
    graphics::{Background::Col, Color, Image},
    lifecycle::{Settings, State, Window, run, Asset},
};

pub use quicksilver::{
    Result, Error,
    combinators::{result, join_all},
    Future,
    load_file,
    geom::Shape,
    graphics::{Background::Img, Font, FontStyle},
    lifecycle::{Event},
    input::{ButtonState, MouseButton, Key},
    sound::Sound,
};

pub use crate::sprites::Sprites;
pub use crate::anim::Animation;
pub use crate::crab::{Crab, Register};
pub use crate::game::Game;
