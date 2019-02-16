use crate::prelude::*;

const ORIGIN_X: f32 = 220.;
const ORIGIN_Y: f32 = 95.;
const TILE_Y: f32 = 28.;
const TILE_X: f32 = 34.;
const OFFSET: f32 = -12.;

const CURSOR: char = '_';
const TEXT_EDITOR_X: f32 = 15.;
const TEXT_EDITOR_Y: f32 = 34.;
const LINE_HEIGHT: f32 = 9.;
const CHAR_WIDTH: f32 = 5.;
const MAX_LEN: usize = 2 << 3;

const REG_Y: f32 = 250.;
const REG_X: f32 = 14.;

const REG_OFFSET: f32 = 20.;

const MAX_LINES: usize = 24;

pub struct Game {
    crab: Crab,
    buf: String,
    is_debugging: bool,
    done: bool,
}

impl Game {
    pub fn new() -> Self {
        let crab = Crab::new();
        let buf = CURSOR.to_string();
        Self {
            crab,
            buf,
            is_debugging: false,
            done: false,
        }
    }

    pub fn char(&mut self, c: char) {
        if self.is_debugging { return }
        // backspace
        if c == '\0' {
            let idx = self.buf.find(CURSOR).unwrap();
            if idx != 0 {
                self.buf.remove(idx-1);
            }
            return;
        }
        // cannot be greater than max line length
        if let Some(len) = self.buf.lines().last().map(|i|i.len()) {
            if len + 1 == MAX_LEN {
                return;
            }
        }
        if c == '\n' {
            // disallow double newline
            let idx = self.buf.find(CURSOR).unwrap();
            if idx == 0 {
                return;
            }
            let prev = &self.buf[(idx-1)..idx];
            if prev == "\n" || prev == " " {
                return;
            }

            // check for max number of line
            if self.buf.lines().collect::<Vec<_>>().len() + 1 > MAX_LINES { return; }
        }
        self.buf = self.buf.replace(CURSOR, &format!("{}{}", c, CURSOR));
    }

    pub fn cursor_left(&mut self) {
        let idx = self.buf.find(CURSOR).unwrap();
        if idx != 0 {
            let ch = self.buf.remove(idx-1);
            self.buf.insert(idx, ch);
        }
    }

    pub fn cursor_right(&mut self) {
        let idx = self.buf.find(CURSOR).unwrap();
        if idx != self.buf.len() - 1 {
            let ch = self.buf.remove(idx);
            self.buf.insert(idx+1, ch);
        }
    }
}

impl Game {
    pub fn draw(&self, window: &mut Window, sprites: &mut Asset<Sprites>) -> Result<()> {
        self.draw_crab(window, sprites)?;
        self.draw_text(window, sprites)?;
        self.draw_registers(window, sprites)?;
        self.draw_debugger(window, sprites)?;
        Ok(())
    }

    fn draw_debugger(&self, window: &mut Window, sprites: &mut Asset<Sprites>) -> Result<()> {
        if !self.is_debugging{ return Ok(()) }
        let ip = self.crab.ip;
        let loc = (TEXT_EDITOR_X - 8., TEXT_EDITOR_Y + LINE_HEIGHT * ip as f32);
        sprites.execute(|spr|{

            // let pointer = spr.get_img("pointer").unwrap();
            // window.draw_ex(&
            //     pointer.area().with_center(loc),
            //     Img(&pointer),
            //     Transform::scale(Vector::new(1., 1.)),
            //     2,
            // );

            let col = if self.done {
                Color{r:22./255., g:94./255., b:0./255., a:255./255.}
            } else {
                Color{r:255./255., g:221./255., b:0./255., a:255./255.}
            };
            window.draw_ex(&
                Rectangle::new(
                    (TEXT_EDITOR_X - 8., TEXT_EDITOR_Y + LINE_HEIGHT * ip as f32 - LINE_HEIGHT * 0.5),
                    (100.-7., LINE_HEIGHT)
                ),
                Col(col),
                Transform::scale(Vector::new(1., 1.)),
                1,
            );

            Ok(())
        })?;
        Ok(())
    }

    fn draw_registers(&self, window: &mut Window, sprites: &mut Asset<Sprites>) -> Result<()> {
        let regs = [Register::A, Register::M, Register::H, Register::V, Register::R];
        for (i, reg) in regs.iter().enumerate() {
            let loc = (REG_X + REG_OFFSET * i as f32, REG_Y);
            let val = self.crab.get_reg(*reg);
            sprites.execute(|spr|{
                let img = spr.render_str(&format!("{:?}{}", reg, val));
                window.draw_ex(&
                    img.area().with_center(loc),
                    Img(&img),
                    Transform::scale(Vector::new(0.1, 0.1)),
                    0,
                );
                Ok(())
            })?;
        }
        Ok(())
    }

    fn draw_crab(&self, window: &mut Window, sprites: &mut Asset<Sprites>) -> Result<()> {
        let crabloc = (
            ORIGIN_X + self.crab.pos_x as f32 * TILE_X + self.crab.pos_y as f32 * OFFSET,
            ORIGIN_Y + self.crab.pos_y as f32 * TILE_Y,
        );
        let crab_normal = self.crab.get_reg(Register::R);
        sprites.execute(|spr|{
            let crab = spr.get_img("crab").unwrap();
            window.draw_ex(&
                crab.area().with_center(crabloc),
                Img(&crab),
                Transform::scale(Vector::new(1, 1)) * Transform::rotate(90 * crab_normal),
                2,
            );
            Ok(())
        })?;
        Ok(())
    }

    fn draw_text(&self, window: &mut Window, sprites: &mut Asset<Sprites>) -> Result<()> {
        sprites.execute(|spr|{
            let mut row = 0.;
            let mut col = 0.;
            for i in self.buf.chars() {
                if i == ' ' {
                    col += 1.;
                    continue;
                } else if i == '\n' {
                    row += 1.;
                    col = 0.;
                    continue;
                }
                let ltr = spr.render_str(&format!("{}", i));
                let loc = (TEXT_EDITOR_X + CHAR_WIDTH * col, TEXT_EDITOR_Y + LINE_HEIGHT * row);
                window.draw_ex(&
                    ltr.area().with_center(loc),
                    Img(&ltr),
                    Transform::scale(Vector::new(0.1, 0.1)),
                    2,
                );
                col += 1.;
            }
            Ok(())
        })?;
        Ok(())
    }
}

impl Game {
    fn load_code(&mut self) {
        let code = self.buf.replace(CURSOR, "");
        self.crab.load_code(&code);
    }

    pub fn step(&mut self) {
        if !self.is_debugging {
            self.is_debugging = true;
            self.load_code();
            return;
        }
        self.crab.motor();
        if let Err(_) = self.crab.step() {
            self.done = true;
        }
        self.crab.sensor();
    }

    pub fn stop(&mut self) {
        self.is_debugging = false;
        self.crab.reset();
        self.done = false;
    }
    pub fn play(&mut self) {
    }
}