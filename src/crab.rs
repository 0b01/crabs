use crate::game::Game;
use std::collections::HashMap;
use std::str::FromStr;

const INIT_X: i32 = 3;
const INIT_Y: i32 = 3;

#[derive(Debug)]
pub struct Crab {
    pub registers: HashMap<Register, i32>,
    /// instruction pointer
    pub ip: usize,
    pub code: Vec<OpCode>,
    pub pos_y: i32,
    pub pos_x: i32,
}

impl Crab {
    pub fn new() -> Self {
        let mut registers = HashMap::new();
        registers.insert(Register::A, 0);
        registers.insert(Register::M, 0);
        registers.insert(Register::H, 0);
        registers.insert(Register::V, 0);
        registers.insert(Register::R, 0);
        Self {
            registers,
            ip: 0,
            code: vec![],
            pos_x: INIT_X,
            pos_y: INIT_Y,
        }
    }

    pub fn load_code(&mut self, code: &str) -> Result<(), usize> {
        let lines: Vec<_> = code.lines()
            .map(|line|line.trim().parse::<OpCode>().ok())
            .collect();
        if !lines.iter().all(|i|i.is_some()) {
            return Err(lines.iter().position(|i|i.is_none()).unwrap());
        }

        let lines = lines.into_iter().map(|i|i.unwrap()).collect();
        self.code = lines;
        Ok(())
    }

    /// returns the unit vector in the direction of freedom
    ///
    /// Note: crabs walk sideways so the degree of freedom = 1 and the
    /// vector corresponds to the complement axis of where the crab
    /// is actually facing
    ///
    ///
    /// 0 v : the crab's right is (x, y) = (-1, 0)
    /// 1 < : the crab's right is (x, y) = (0, -1)
    /// 2 ^ : the crab's right is (x, y) = (1, 0)
    /// 3 > : the crab's right is (x, y) = (0, 1)
    pub fn dir(&self) -> (i32, i32) {
        match self.get_reg(Register::R) {
            0 => (-1, 0),
            1 => (0, -1),
            2 => (1, 0),
            3 => (0, 1),
            _ => panic!("impossible.")
        }

    }

    pub fn motor(&mut self) {
        if self.stopped() { return; }
        let m = self.get_reg(Register::M);
        let to_move = m.signum();
        let (x, y) = self.dir();
        self.pos_x += x * to_move;
        self.pos_y += y * to_move;
    }

    pub fn sensor(&mut self/*, game: &Game*/) {
        // ...
    }

    fn find_label(&self, to_find: &str) -> Option<usize> {
        for (i, op) in self.code.iter().enumerate() {
            if let OpCode::LABEL(lbl) = op {
                if lbl == to_find {
                    return Some(i)
                }
            }
        }
        None
    }

    pub fn get_reg_mut(&mut self, reg: Register) -> Option<&mut i32> {
        if reg == Register::R {
            None
        } else {
            self.registers.get_mut(&reg)
        }
    }

    pub fn get_reg(&self, reg: Register) -> i32 {
        *self.registers.get(&reg).unwrap()
    }

    pub fn reset(&mut self) {
        for i in self.registers.values_mut() {
            *i = 0;
        }
        self.pos_x = INIT_X;
        self.pos_y = INIT_Y;
        self.ip = 0;
    }

    pub fn execute(&mut self) -> Result<(), String> {
        let mut cycles = 0;
        loop {
            cycles += 1;
            self.step()?;
            if self.ip >= self.code.len() {
                break;
            }
            if cycles > 1_000_000 {
                return Err("executed more than 1000000 cycles".to_owned())
            }
        }

        Ok(())
    }

    pub fn stopped(&self) -> bool {
        self.ip >= self.code.len()
    }

    pub fn step(&mut self) -> Result<(), String> {
        if self.stopped() {
            return Err("Runtime error: instruction pointer OOB".to_owned())
        }

        dbg!(&self);
        dbg!(&self.code[self.ip]);

        let op = self.code[self.ip].clone();

        match &op {
            LABEL(_) => (),
            COMMENT(_) => (),
            _ => self.motor(),
        };

        use self::OpCode::*;
        match op {
            NOP => {
                self.ip += 1;
            },
            LABEL(lbl) => {
                self.ip += 1;
                return Ok(());
            },
            COMMENT(_) => {
                self.ip += 1;
            },
            MOVI(num, reg) => {
                if let Some(reg) = self.get_reg_mut(reg) {
                    *reg = num;
                }
                self.ip += 1;
            }
            MOV(reg1, reg2) => {
                let val = self.get_reg(reg1);
                if let Some(reg) = self.get_reg_mut(reg2) {
                    *reg = val;
                }
                self.ip += 1;
            }
            ADDI(num, reg) => {
                if let Some(reg) = self.get_reg_mut(reg) {
                    *reg += num;
                }
                self.ip += 1;
            }
            ADD(reg1, reg2) => {
                let val = self.get_reg(reg1);
                if let Some(reg) = self.get_reg_mut(reg2) {
                    *reg += val;
                }
                self.ip += 1;
            }
            SUBI(num, reg) => {
                if let Some(reg) = self.get_reg_mut(reg) {
                    *reg -= num;
                }
                self.ip += 1;
            }
            SUB(reg1, reg2) => {
                let val = self.get_reg(reg1);
                if let Some(reg) = self.get_reg_mut(reg2) {
                    *reg -= val;
                }
                self.ip += 1;
            }
            NEG(reg) => {
                let val = -self.get_reg(reg);
                if let Some(reg) = self.get_reg_mut(reg) {
                    *reg = val;
                }
                self.ip += 1;
            }
            JMP(lbl) => {
                let loc = self.find_label(&lbl).ok_or(format!("Cannot find label. {}", lbl))?;
                dbg!(loc);
                self.ip = loc;
            }
            JEZ(lbl) => {
                let loc = self.find_label(&lbl).ok_or(format!("Cannot find label. {}", lbl))?;
                if self.get_reg(Register::A) == 0 {
                    self.ip = loc;
                } else {
                    self.ip += 1;
                }
            }
            JNZ(lbl) => {
                let loc = self.find_label(&lbl).ok_or(format!("Cannot find label. {}", lbl))?;
                if self.get_reg(Register::A) != 0 {
                    self.ip = loc;
                } else {
                    self.ip += 1;
                }
            }
            JGZ(lbl) => {
                let loc = self.find_label(&lbl).ok_or(format!("Cannot find label. {}", lbl))?;
                if self.get_reg(Register::A) > 0 {
                    self.ip = loc;
                } else {
                    self.ip += 1;
                }
            }
            // jump to label if acc < 0
            JLZ(lbl) => {
                let loc = self.find_label(&lbl).ok_or(format!("Cannot find label. {}", lbl))?;
                if self.get_reg(Register::A) < 0 {
                    self.ip = loc;
                } else {
                    self.ip += 1;
                }

            }
            // unconditional relative jump
            JROI(num) => {
                let tmp = self.ip as i32 + num;
                if (tmp < 0) || (self.ip >= self.code.len()) {
                    return Err("Cannot jump to that location".to_owned())
                } else {
                    self.ip = tmp as usize;
                }
            }
            // unconditional relative jump with value from register
            JRO(reg) => {
                let num = self.get_reg(reg);
                let tmp = self.ip as i32 + num;
                if (tmp < 0) || (self.ip >= self.code.len()) {
                    return Err("Cannot jump to that location".to_owned())
                } else {
                    self.ip = tmp as usize;
                }
            }
            RCW => {
                let dir = self.registers.get_mut(&Register::R).unwrap();
                *dir = (*dir + 1) % 4;
                self.ip += 1;
            }
            RCC => {
                let dir = self.registers.get_mut(&Register::R).unwrap();
                *dir = (*dir + 3) % 4;
                self.ip += 1;
            }
        };
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum OpCode {
    NOP,
    LABEL(String),
    COMMENT(String),
    MOVI(i32, Register),
    MOV(Register, Register),
    ADDI(i32, Register),
    ADD(Register, Register),
    SUBI(i32, Register),
    SUB(Register, Register),
    NEG(Register),
    /// jump to label
    JMP(String),
    /// jump to label if acc == 0
    JEZ(String),
    /// jump to label if acc != 0
    JNZ(String),
    /// jump to label if acc > 0
    JGZ(String),
    /// jump to label if acc < 0
    JLZ(String),
    /// unconditional relative jump
    JROI(i32),
    /// unconditional relative jump with value from register
    JRO(Register),
    /// rotate clockwise
    RCW,
    /// rotate counterclosewise
    RCC,
}

impl FromStr for OpCode {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<_> = line.split_whitespace().collect();
        let code = tokens.get(0).ok_or("does not exist".to_owned())?;
        let op1 = tokens.get(1).ok_or("does not exist".to_owned());
        let op2 = tokens.get(2).ok_or("does not exist".to_owned());

        match code {
            &"MOV" => {
                if op1.clone()?.chars().all(|i|i.is_digit(10) || i == '-') {
                    let op1 = op1?.parse().map_err(|_| "cannot parse int".to_owned())?;
                    let op2 = op2?.parse()?;
                    Ok(OpCode::MOVI(op1, op2))
                } else {
                    let op1 = op1?.parse()?;
                    let op2 = op2?.parse()?;
                    Ok(OpCode::MOV(op1, op2))
                }
            }
            &"ADD" => {
                if op1.clone()?.chars().all(|i|i.is_digit(10) || i == '-') {
                    let op1 = op1?.parse().map_err(|_| "cannot parse int".to_owned())?;
                    let op2 = op2?.parse()?;
                    Ok(OpCode::ADDI(op1, op2))
                } else {
                    let op1 = op1?.parse()?;
                    let op2 = op2?.parse()?;
                    Ok(OpCode::ADD(op1, op2))
                }
            }
            &"SUB" => {
                if op1.clone()?.chars().all(|i|i.is_digit(10) || i == '-') {
                    let op1 = op1?.parse().map_err(|_| "cannot parse int".to_owned())?;
                    let op2 = op2?.parse()?;
                    Ok(OpCode::SUBI(op1, op2))
                } else {
                    let op1 = op1?.parse()?;
                    let op2 = op2?.parse()?;
                    Ok(OpCode::SUB(op1, op2))
                }
            }
            &"NEG" => {
                let op1 = op1?.parse()?;
                Ok(OpCode::NEG(op1))
            }
            &"NOP" => {
                Ok(OpCode::NOP)
            }
            &"JRO" => {
                if op1.clone()?.chars().all(|i|i.is_digit(10) || i == '-') {
                    let op1 = op1?.parse().map_err(|_| "cannot parse int".to_owned())?;
                    Ok(OpCode::JROI(op1))
                } else {
                    let op1 = op1?.parse()?;
                    Ok(OpCode::JRO(op1))
                }
            }
            &"JMP" => Ok(OpCode::JMP(op1?.to_string())),
            &"JEZ" => Ok(OpCode::JEZ(op1?.to_string())),
            &"JNZ" => Ok(OpCode::JNZ(op1?.to_string())),
            &"JGZ" => Ok(OpCode::JGZ(op1?.to_string())),
            &"JLZ" => Ok(OpCode::JLZ(op1?.to_string())),
            &"RCW" => Ok(OpCode::RCW),
            &"RCC" => Ok(OpCode::RCC),
            _ => {
                if code.starts_with("#") {
                    Ok(OpCode::COMMENT( line.to_string()) )
                } else if code.ends_with(":") {
                    Ok(OpCode::LABEL( code[0..code.len()-1].to_string() ))
                } else {
                    Err("Not a valid instruction!".to_owned())
                }
            }
        }
    }
}


#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum Register {
    /// object distance in horizontal direction
    H,
    /// object distance in vertical direction
    V,
    /// general purpose register
    A,
    /// motor register, will go right if positive, left if negative
    M,
    /// rotation v0 <1 ^2 >3
    R,
}

impl FromStr for Register {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "V" => Ok(Register::V),
            "H" => Ok(Register::H),
            "A" => Ok(Register::A),
            "M" => Ok(Register::M),
            "R" => Ok(Register::R),
            _ => Err("none".to_owned())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_op() {
        let mut crab = Crab::new();
        let code = r#"
            # TEST BLAH
            MOV 1 A
            ADD 1 A
            SUB 1 A
            NEG A
            JRO -1
            JRO A
            LABEL:
            JMP LABEL
        "#;
        crab.load_code(code).unwrap();
    }

    #[test]
    fn test_step() {
        let mut crab = Crab::new();
        let code = r#"
                MOV 1 A
                ADD 1 A
                SUB 1 A
                NEG A
                JRO A
            LABEL:
                SUB 1 A
                JGZ LABEL
        "#;
        crab.load_code(code).unwrap();
        crab.execute().unwrap();
        // crab.step().unwrap();
        // crab.step().unwrap();
        // crab.step().unwrap();
        // crab.step().unwrap();
        // crab.step().unwrap();
        // crab.step().unwrap();
        // crab.step().unwrap();
        // crab.step().unwrap();
        // crab.step().unwrap();
        // crab.step().unwrap();
        // crab.step().unwrap();
        // crab.step().unwrap();
        // crab.step().unwrap();
    }
}
