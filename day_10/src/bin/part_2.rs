use day_10::{Instr, Machine};

fn main() {
    let input = include_str!("../../../input/day_10.txt");

    let observe = [20_usize, 60, 100, 140, 180, 220];
    let mut message = String::with_capacity(256);

    let mut machine = Machine::new(|cycle, reg| {
        let x = (cycle as i32 - 1) % 40;
        if (reg - x).abs() <= 1 {
            message.push('#');
            message.push('#');
        } else {
            message.push('.');
            message.push('.');
        }

        if cycle % 40 == 0 {
            message.push('\n');
        }
    });

    input
        .lines()
        .map(Instr::from)
        .for_each(|instr| machine.run_instruction(instr));

    println!("{}", message);
}
