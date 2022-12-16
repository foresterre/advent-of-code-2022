use day_10::{Instr, Machine};

fn main() {
    let input = include_str!("../../../input/day_10.txt");

    let observe = [20_usize, 60, 100, 140, 180, 220];
    let mut signal = 0;

    let mut machine = Machine::new(|cycle, reg| {
        if observe.contains(&cycle) {
            signal += (cycle as i32) * reg as i32;
        }
    });

    let mut instructions = input.lines().map(Instr::from);

    while !(machine.cycle() > 220) {
        let next = instructions.next().unwrap();
        machine.run_instruction(next);
    }

    println!("summed: {}", signal);
}
