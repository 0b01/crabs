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
const MAX_LINES: usize = 24;

const REG_Y: f32 = 250.;
const REG_X: f32 = 14.;
const REG_OFFSET: f32 = 20.;

/// continuous stepping delay time in ms
const PLAY_DELAY: f64 = 100.;

const GRID_W: usize = 8;
const GRID_H: usize = 6;

type Grid = Vec<Vec<bool>>;

pub struct Game {
    crab: Crab,
    buf: String,
    is_debugging: bool,
    is_playing: bool,
    code_finished: bool,
    sleep: f64,
    error: Option<usize>,
    current_grid: Grid,
    current_level: usize,
    levels: Vec<Grid>,
}

impl Game {
    fn init_levels() -> Vec<Grid> {
        let mut ret = vec![];

        ret.push(to_grid(include_str!("levels/tutorial.txt")));
        ret.push(to_grid(include_str!("levels/tutorial2.txt")));
        ret.push(to_grid(include_str!("levels/tutorial3.txt")));
        ret.push(to_grid(include_str!("levels/tutorial4.txt")));
        ret.push(to_grid(include_str!("levels/2.txt")));
        ret.push(to_grid(include_str!("levels/1.txt")));
        ret.push(to_grid(include_str!("levels/test.txt")));

        ret
    }

    pub fn new() -> Self {
        let crab = Crab::new();
        let buf = CURSOR.to_string();
        let levels = Game::init_levels();
        Self {
            crab,
            buf,
            is_debugging: false,
            is_playing: false,
            code_finished: false,
            sleep: 0.,
            error: None,
            current_level: 0,
            current_grid: levels[0].clone(),
            levels,
        }
    }

    pub fn update(&mut self, window: &mut Window, sprites: &mut Asset<Sprites>) -> Result<()>  {
        let rate = window.update_rate();
        self.sleep -= rate;
        if self.sleep < 0. && self.is_playing && !self.code_finished {
            self.step(sprites);
            self.sleep = PLAY_DELAY;
            if self.code_finished {
                self.is_playing = false;
            }
        }
        Ok(())
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
            // check number of lines
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
        self.draw_grid_items(window, sprites)?;
        self.draw_text(window, sprites)?;
        self.draw_registers(window, sprites)?;
        self.draw_debugger(window, sprites)?;
        self.draw_error(window, sprites)?;
        self.draw_level(window, sprites)?;
        Ok(())
    }

    fn draw_level(&self, window: &mut Window, sprites: &mut Asset<Sprites>) -> Result<()> {
        sprites.execute(|spr|{
            let img = spr.render_str(&format!("Level: {}/{}", self.current_level + 1, self.levels.len()));
            window.draw_ex(&
                img.area().with_center((448., 250.)),
                Img(&img),
                Transform::scale(Vector::new(0.1, 0.1)),
                1,
            );
            Ok(())
        })?;
        Ok(())
    }

    fn draw_grid_items(&self, window: &mut Window, sprites: &mut Asset<Sprites>) -> Result<()> {
        for (i, row) in self.current_grid.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {

                let crabloc = (
                    ORIGIN_X + j as f32 * TILE_X + i as f32 * OFFSET,
                    ORIGIN_Y + i as f32 * TILE_Y,
                );

                if *col {
                    sprites.execute(|spr|{
                        let crab = spr.get_anim("small").unwrap().current_frame();
                        window.draw_ex(&
                            crab.area().with_center(crabloc),
                            Img(&crab),
                            Transform::scale(Vector::new(1, 1)),//* Transform::rotate(90 * crab_normal),
                            2,
                        );
                        Ok(())
                    })?;
                }
            }
        }
        Ok(())
    }

    fn draw_error(&self, window: &mut Window, sprites: &mut Asset<Sprites>) -> Result<()> {
        if let Some(line) = self.error {
            window.draw_ex(&
                Rectangle::new(
                    (TEXT_EDITOR_X - 8., TEXT_EDITOR_Y + LINE_HEIGHT * line as f32 - LINE_HEIGHT * 0.5),
                    (100.-7., LINE_HEIGHT)
                ),
                Col(Color{r:0./255., g:0./255., b:255./255., a:255./255.}),
                Transform::scale(Vector::new(1., 1.)),
                1,
            );
        }
        Ok(())
    }
    fn draw_debugger(&self, window: &mut Window, sprites: &mut Asset<Sprites>) -> Result<()> {
        if !self.is_debugging{ return Ok(()) }
        let ip = self.crab.ip;
        let loc = (TEXT_EDITOR_X - 8., TEXT_EDITOR_Y + LINE_HEIGHT * ip as f32);
        sprites.execute(|spr|{

            let pointer = spr.get_img("pointer").unwrap();
            window.draw_ex(&
                pointer.area().with_center(loc),
                Img(&pointer),
                Transform::scale(Vector::new(1., 1.)),
                2,
            );

            let col = if self.code_finished {
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
                let img = spr.render_str(&format!("{:?}:{}", reg, val));
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
        let anim_name = match crab_normal {
            0 => "crab-rest",
            1 => "crab-left",
            2 => "crab-up",
            3 => "crab-right",
            _ => panic!("impossible"),
        };
        sprites.execute(|spr|{
            let crab = spr.get_anim(anim_name).unwrap().current_frame();
            window.draw_ex(&
                crab.area().with_center(crabloc),
                Img(&crab),
                Transform::scale(Vector::new(1, 1)),//* Transform::rotate(90 * crab_normal),
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
        if let Err(line) = self.crab.load_code(&code) {
            self.error = Some(line);
        }
    }

    fn objective_completed(&self) -> bool {
        for i in &self.current_grid {
            for j in i {
                if *j { return false; }
            }
        }
        return true;
    }

    fn next_level(&mut self, sprites: &mut Asset<Sprites>) {
        self.code_finished = true;
        self.current_level += 1;
        self.current_grid = self.levels[self.current_level].clone();
        self.stop();
        sprites.execute(|i| {
            i.get_sound("success").unwrap().play()?;
            Ok(())
        }).unwrap();
    }

    pub fn step(&mut self, sprites: &mut Asset<Sprites>) {
        if self.objective_completed() {
            self.next_level(sprites);
        }
        if !self.is_debugging {
            self.is_debugging = true;
            self.load_code();
            return;
        }
        if self.error.is_some()  {
            return;
        }
        if let Err(_) = self.crab.step() {
            self.code_finished = true;
        }
        let i = self.crab.pos_y as usize;
        let j = self.crab.pos_x as usize;
        if let Some(Some(pos)) = self.current_grid.get_mut(i).map(|row|row.get_mut(j)) {
            *pos = false;
        }
        self.crab.sensor();
    }

    pub fn stop(&mut self) {
        self.is_debugging = false;
        self.is_playing = false;
        self.crab.reset();
        self.current_grid = self.levels[self.current_level].clone();
        self.error = None;
        self.code_finished = true;
    }

    pub fn play(&mut self) {
        self.is_playing = true;
        self.code_finished = false;
    }
}

fn to_grid(file: &str) -> Grid {
    let mut grid = vec![];
    for line in file.lines().take(GRID_H) {
        let mut temp = vec![];
        for ch in line.chars().take(GRID_W) {
            temp.push(ch=='x');
        }
        grid.push(temp);
    }
    grid
}