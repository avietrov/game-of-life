use std::fmt;

#[derive(PartialEq, Eq, Clone, Copy)]
enum CellState {
    Dead,
    Alive,
}

#[derive(PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

struct Generation {
    size: usize,
    state: Vec<CellState>,
}

impl Generation {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            state: vec![CellState::Dead; size * size],
        }
    }
}

impl Position {
    fn neighbours(&self) -> Vec<Position> {
        let x = self.x as i32;
        let y = self.y as i32;
        let points: Vec<(i32, i32)> = vec![
            (x - 1, y - 1),
            (x, y - 1),
            (x + 1, y - 1),
            (x - 1, y),
            (x + 1, y),
            (x - 1, y + 1),
            (x, y + 1),
            (x + 1, y + 1),
        ];

        points
            .iter()
            .filter(|(x, y)| *x >= 0 && *y >= 0)
            .map(|(x, y)| Position {
                x: (*x as usize),
                y: (*y as usize),
            })
            .collect()
    }
}

impl Generation {
    fn index(&self, p: &Position) -> usize {
        p.x * self.size + p.y
    }

    fn position(&self, i: usize) -> Position {
        Position {
            x: i / self.size,
            y: i % self.size,
        }
    }

    fn get(&self, p: &Position) -> CellState {
        let index = self.index(p);
        if index < 0 || index >= self.state.len() {
            CellState::Dead
        } else {
            self.state[index]
        }
    }

    fn set(&mut self, p: &Position, s: CellState) {
        let index = self.index(p);
        self.state[index] = s
    }

    fn alive_neighbours(&self, p: &Position) -> usize {
        p.neighbours()
            .iter()
            .filter(|n| self.get(n) == CellState::Alive)
            .count()
    }

    fn evolve(&self) -> Generation {
        let state = (0..(self.size * self.size))
            .into_iter()
            .map(|index| self.position(index))
            .map(|pos| match self.alive_neighbours(&pos) {
                3 => CellState::Alive,
                2 => self.get(&pos),
                _ => CellState::Dead,
            })
            .collect();

        Generation {
            size: self.size,
            state: state,
        }
    }
}

impl fmt::Display for Generation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let bar: String = String::from("╔") + &"═".repeat(self.size) + &String::from("╗");
        let mut str = bar + "\n";
        for x in 0..self.size {
            str += "║";
            for y in 0..self.size {
                let index = x * self.size + y;
                let cell = match self.state[index] {
                    CellState::Alive => "x",
                    CellState::Dead => " ",
                };
                str = str + cell
            }
            str = str + "║\n"
        }
        let bar2 = String::from("╚") + &"═".repeat(self.size) + &String::from("╝");
        str += &bar2;

        write!(f, "{}", str)
    }
}

fn main() {
    let mut init = Generation::new(10);
    init.set(&Position { x: 2, y: 3 }, CellState::Alive);
    init.set(&Position { x: 3, y: 3 }, CellState::Alive);
    init.set(&Position { x: 4, y: 3 }, CellState::Alive);

    for i in 0..10 {
        println!("{} \n{}", i, init);
        init = init.evolve()
    }
}
