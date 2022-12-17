fn main() {
    let input = include_str!("../../../input/example/day_09.txt");

    let mut head = Head { x: 0, y: 0 };

    for (dir, amount) in input.lines().map(|line| {
        let (dir, amount) = line.split_once(' ').unwrap();

        (Direction::from(dir), amount.parse::<i32>().unwrap())
    }) {
        head.move_by(dir, amount);
    }
}

struct Head {
    x: i32,
    y: i32,
}

impl Head {
    pub fn move_by(&mut self, dir: Direction, amount: i32) {
        for _ in 0..amount {
            let (x, y) = Self::relative(dir);
            self.x += x;
            self.y += y;
        }

        println!("Now at: {}, {}", self.x, self.y);
    }

    fn relative(dir: Direction) -> (i32, i32) {
        match dir {
            Direction::R => (1, 0),
            Direction::L => (-1, 0),
            Direction::U => (0, 1),
            Direction::D => (0, -1),
        }
    }
}

struct Tail {
    x: i32,
    y: i32,
}

#[derive(Copy, Clone)]
enum Direction {
    R,
    L,
    U,
    D,
}

impl<'s> From<&'s str> for Direction {
    fn from(input: &'s str) -> Self {
        match input.chars().next().unwrap() {
            'R' => Self::R,
            'L' => Self::L,
            'U' => Self::U,
            'D' => Self::D,
            _ => unreachable!(),
        }
    }
}
