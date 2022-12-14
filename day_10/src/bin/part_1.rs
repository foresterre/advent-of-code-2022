use either::Either;
use std::collections::BTreeMap;

#[derive(Copy, Clone, Debug)]
enum Instr {
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

struct Machine {
    register: i32,
    cycle: usize,
    observe_cycles: BTreeMap<usize, Option<i32>>,
}

impl Machine {
    fn new(cycles: impl IntoIterator<Item = usize>) -> Self {
        Self {
            register: 1,
            cycle: 0,
            observe_cycles: cycles.into_iter().map(|c| (c, None)).collect(),
        }
    }
}

impl Machine {
    /// Runs the instruction on the machine, and returns the value of the
    /// register prior to and after
    pub fn run_instruction(&mut self, instr: Instr) {
        match instr {
            Instr::Noop => {
                self.cycle += 1;
                self.observe();
            }
            Instr::Addx(v) => {
                self.cycle += 1;
                self.observe();

                self.cycle += 1;
                self.observe();

                self.register += v;
            }
        }
    }

    fn observe(&mut self) {
        let current = self.cycle;

        if self.observe_cycles.contains_key(&current) {
            self.observe_cycles.insert(current, Some(self.register));
        }
    }

    pub fn cycle(&self) -> usize {
        self.cycle
    }

    pub fn observations(&self) -> &BTreeMap<usize, Option<i32>> {
        &self.observe_cycles
    }
}

fn main() {
    let input = include_str!("../../../input/day_10.txt");

    // TODO: this does somehow not work, I'm so saddd :).
    //  For the example, only the last value does not match and is off by one (19, vs.
    //  the apparently correct 18). My guess would be that on the second cycle of the addx
    //  the value must be unchanged, and only after the second cycle, it would update.
    //  ...
    // // Parse input instructions, and lower them to one cycle variants.
    // let iter = input
    //     .lines()
    //     .map(Instr::from)
    //     .flat_map(|instr| match instr {
    //         // Map all instructions is such a way that each instruction represents a single cycle.
    //         // For the 1-cycle Noop instruction, we keep it as-is.
    //         // For the 2-cycle Addx, we lower it to a 1-cycle Noop and a 1-cycle Addx instruction.
    //         //  That is, after the flatten step we consider Addx to be 1-cycle too.
    //         Instr::Noop => Either::Left(iter::once(Instr::Noop)),
    //         Instr::Addx(v) => Either::Right([Instr::Noop, Instr::Addx(v)].into_iter()),
    //     })
    //     .map(|instr| {
    //         println!("{:?}", instr);
    //         match instr {
    //             // Map the instructions to their register value, so we can sum these to get the total
    //             // value
    //             Instr::Noop => 0,
    //             Instr::Addx(v) => v,
    //         }
    //     });
    //
    // let sum = (20..=220)
    //     .step_by(40)
    //     .map(|amount| {
    //         // + 1 is because the register starts at 1, not 0.
    //         let mut sum = iter.clone().take(amount).sum::<i32>() + 1;
    //         // return the signal strength
    //         let signal_strength = sum * (amount as i32);
    //
    //         println!("on '{amount}', the register has value '{sum}' and the signal strength is '{signal_strength}'");
    //
    //         signal_strength
    //     })
    //     .sum::<i32>();

    let mut machine = Machine::new([220usize, 180, 140, 100, 60, 20]);
    let mut instructions = input.lines().map(Instr::from);

    while !(machine.cycle() > 220) {
        let next = instructions.next().unwrap();
        machine.run_instruction(next);
    }

    let sum = machine
        .observations()
        .iter()
        .map(|(cycle, value)| {
            let value = value.unwrap();
            let sum = (*cycle as i32) * value;

            println!("cycle = {}, value = {}, sum = {}", cycle, value, sum);

            sum
        })
        .sum::<i32>();

    println!("summed: {}", sum);
}
