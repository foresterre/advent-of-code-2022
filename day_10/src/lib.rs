#[derive(Copy, Clone, Debug)]
pub enum Instr {
    Addx(i32),
    Noop,
}

impl<'s> From<&'s str> for Instr {
    fn from(input: &'s str) -> Self {
        match &input[..=3] {
            "addx" => Self::Addx((&input[5..]).parse().unwrap()),
            "noop" => Self::Noop,
            _ => unreachable!(),
        }
    }
}

pub struct Machine<Observer: FnMut(usize, i32)> {
    register: i32,
    cycle: usize,
    observer: Observer,
}

impl<Observer: FnMut(usize, i32)> Machine<Observer> {
    pub fn new(observer: Observer) -> Self {
        Self {
            register: 1,
            cycle: 0,
            observer,
        }
    }

    /// Runs the instruction on the machine, and returns the value of the
    /// register prior to and after
    pub fn run_instruction(&mut self, instr: Instr) {
        match instr {
            Instr::Noop => {
                self.tick();
            }
            Instr::Addx(v) => {
                self.tick();
                self.tick();
                self.register += v;
            }
        }
    }

    pub fn cycle(&self) -> usize {
        self.cycle
    }

    fn tick(&mut self) {
        self.cycle += 1;
        (self.observer)(self.cycle, self.register);
    }
}
